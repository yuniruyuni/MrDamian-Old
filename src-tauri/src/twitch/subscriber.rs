use futures::StreamExt;
use twitch_api::{
    eventsub::{
        Event, EventsubWebsocketData, Message as TwitchMessage, NotificationMetadata, Payload,
        ReconnectPayload, WelcomePayload,
    },
    helix::HelixClient,
    twitch_oauth2::{AccessToken, UserToken},
    types::{UserId, UserName},
};

use async_trait::async_trait;
use miette::{miette, IntoDiagnostic, Result, WrapErr};

use crate::error::MrDamianError;
use crate::pipeline::{Component, Connection, Message, Packet, PassiveComponent, Property};

type WSConnection =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct Subscriber {
    client: HelixClient<'static, reqwest::Client>,
    oauth: AccessToken,
    token: Option<UserToken>,

    channel: UserName,
    channel_id: UserId,

    bot: UserName,
    bot_id: UserId,

    session_id: Option<String>,
    reconnect_url: Option<url::Url>,

    socket: Option<WSConnection>,

    conn: Connection,
}

impl Subscriber {
    pub fn new(bot: &str, channel: &str, oauth: &str) -> Self {
        let client: HelixClient<reqwest::Client> = HelixClient::default();

        Self {
            client,
            socket: None,
            channel: channel.into(),
            bot: bot.into(),
            oauth: oauth.into(),
            token: None,
            session_id: None,
            reconnect_url: None,
            channel_id: "".into(),
            bot_id: "".into(),
            conn: Connection::new(),
        }
    }

    pub async fn get_user_id_for(&mut self, name: &UserName) -> Result<UserId> {
        let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;
        self.client
            .get_user_from_login(name, token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    async fn connect(&mut self) -> Result<WSConnection> {
        let config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default();

        let (socket, _) = tokio_tungstenite::connect_async_with_config(
            twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            Some(config),
        )
        .await
        .into_diagnostic()
        .wrap_err("Cannot connect twitch event server host")?;

        Ok(socket)
    }

    async fn process_message(&mut self, msg: String) -> Result<()> {
        use EventsubWebsocketData::*;
        match Event::parse_websocket(msg.as_str()).into_diagnostic()? {
            Welcome {
                payload: WelcomePayload { session },
                ..
            }
            | Reconnect {
                payload: ReconnectPayload { session },
                ..
            } => {
                self.channel_id = self.get_user_id_for(&self.channel.clone()).await?;
                self.bot_id = self.get_user_id_for(&self.bot.clone()).await?;

                self.session_id = Some(session.id.to_string());
                if let Some(url) = session.reconnect_url {
                    self.reconnect_url = Some(url.parse().into_diagnostic()?);
                }

                let req = twitch_api::helix::eventsub::CreateEventSubSubscriptionRequest::default();
                let body = twitch_api::helix::eventsub::CreateEventSubSubscriptionBody::new(
                    twitch_api::eventsub::channel::ChannelRaidV1::to_broadcaster_user_id(
                        self.channel_id.clone(),
                    ),
                    twitch_api::eventsub::Transport::websocket(session.id.to_string()),
                );

                let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;
                self.client
                    .req_post(req, body, token)
                    .await
                    .into_diagnostic()?;
            }
            Notification { metadata, payload } => {
                self.process_notification(&metadata, &payload).await?;
            }
            Revocation { .. } => (),
            Keepalive { .. } => (),
            _ => (),
        }

        Ok(())
    }

    async fn process_notification<'a>(
        &mut self,
        _metadata: &NotificationMetadata<'a>,
        payload: &Event,
    ) -> Result<()> {
        match payload {
            Event::ChannelRaidV1(Payload {
                message: TwitchMessage::Notification(msg),
                ..
            }) => {
                let mut message = Message::new();
                message.insert("event".to_string(), Property::Text("raid".to_string()));
                message.insert(
                    "from_broadcaster_user_id".to_string(),
                    Property::Text(msg.from_broadcaster_user_id.to_string()),
                );
                message.insert(
                    "from_broadcaster_user_login".to_string(),
                    Property::Text(msg.from_broadcaster_user_login.to_string()),
                );
                message.insert(
                    "from_broadcaster_user_name".to_string(),
                    Property::Text(msg.from_broadcaster_user_name.to_string()),
                );
                message.insert(
                    "to_broadcaster_user_id".to_string(),
                    Property::Text(msg.to_broadcaster_user_id.to_string()),
                );
                message.insert(
                    "to_broadcaster_user_login".to_string(),
                    Property::Text(msg.to_broadcaster_user_login.to_string()),
                );
                message.insert(
                    "to_broadcaster_user_name".to_string(),
                    Property::Text(msg.to_broadcaster_user_name.to_string()),
                );
                message.insert("viewers".to_string(), Property::I64(msg.viewers));
                self.conn
                    .send(Packet {
                        port: "raid".to_string(),
                        message,
                    })
                    .await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl Component for Subscriber {
    fn name(&self) -> String {
        "TwitchSubscriber".to_string()
    }

    fn connection(&mut self) -> &mut Connection {
        &mut self.conn
    }

    async fn run(&mut self) -> Result<()> {
        self.default_run().await
    }
}

#[async_trait]
impl PassiveComponent for Subscriber {
    async fn setup(&mut self) -> Result<()> {
        let token = UserToken::from_token(&self.client, self.oauth.clone())
            .await
            .into_diagnostic()?;
        self.token = Some(token);
        self.socket = Some(self.connect().await?);
        Ok(())
    }

    async fn handler(&mut self) -> Result<Vec<Packet>> {
        use tokio_tungstenite::tungstenite::*;

        let socket = self.socket.as_mut().ok_or(MrDamianError::InvalidSocket)?;
        match socket.next().await {
            Some(Ok(Message::Text(msg))) => {
                if let Err(err) = self.process_message(msg).await {
                    eprintln!("message process error: {}", err);
                }
            }
            Some(err @ Err(Error::ConnectionClosed)) => {
                // but if twitch says you should close connection, we want to along with that.
                err.into_diagnostic()
                    .wrap_err("Twitch connection was closed.")?;
            }
            None
            | Some(Err(Error::Protocol(error::ProtocolError::ResetWithoutClosingHandshake))) => {
                self.socket = Some(self.connect().await?);
            }
            _ => (),
        }
        Ok(vec![])
    }
}

use crate::pipeline::Constructor;
use crate::protocol::{Assign, InputPort, OutputPort};

#[derive(Debug, Default, Clone)]
pub struct SubscriberFactory {}

impl Constructor for SubscriberFactory {
    fn component_type(&self) -> String {
        "TwitchSubscriber".to_string()
    }
    fn construct(&self, config: &crate::config::Config) -> Box<dyn Component + Send> {
        Box::new(Subscriber::new(&config.bot, &config.channel, &config.token))
    }
    fn label(&self) -> String {
        "Twitch Subscriber".to_string()
    }
    fn inputs(&self) -> Vec<InputPort> {
        vec![]
    }
    fn outputs(&self) -> Vec<OutputPort> {
        vec![OutputPort {
            name: "raid".to_string(),
            assign: {
                let mut h = Assign::new();
                h.insert("from_broadcaster_user_id".to_string(), "".to_string());
                h.insert("from_broadcaster_user_login".to_string(), "".to_string());
                h.insert("from_broadcaster_user_name".to_string(), "".to_string());
                h.insert("to_broadcaster_user_id".to_string(), "".to_string());
                h.insert("to_broadcaster_user_login".to_string(), "".to_string());
                h.insert("to_broadcaster_user_name".to_string(), "".to_string());
                h.insert("viewers".to_string(), "".to_string());
                h
            },
        }]
    }
}
