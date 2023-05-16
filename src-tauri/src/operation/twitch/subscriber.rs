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

use crate::operation::pipeline::{
    Component, Connection, Constructor, Message, Packet, Process, ProcessInit, Property,
};
use crate::{
    model::{InputPort, OutputPort, OutputPortID},
    operation::pipeline::PassiveProcess,
};

type WSConnection =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

#[derive(Debug, Clone)]
pub struct SubscriberComponent {
    id: String,
    oauth: AccessToken,
    channel: UserName,
}

impl SubscriberComponent {
    pub fn constructor() -> Constructor {
        Constructor {
            kind: "TwitchSubscriber",
            label: "Twitch Subscriber",
            gen: Box::new(
                |id: &str, config: &crate::config::Config| -> Box<dyn Component + Send> {
                    Box::new(SubscriberComponent::new(
                        id,
                        &config.channel,
                        &config.token,
                    ))
                },
            ),
        }
    }

    pub fn new(id: &str, channel: &str, oauth: &str) -> Self {
        Self {
            id: id.to_string(),
            channel: channel.into(),
            oauth: oauth.into(),
        }
    }
}

impl Component for SubscriberComponent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn kind(&self) -> &'static str {
        "TwitchSubscriber"
    }

    fn label(&self) -> &'static str {
        "Twitch Subscriber"
    }

    fn inputs(&self) -> Vec<InputPort> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputPort> {
        vec![OutputPort {
            id: OutputPortID {
                parent: self.id.clone(),
                name: "raid".to_string(),
            },
            property_names: vec![
                "from_broadcaster_user_id".to_string(),
                "from_broadcaster_user_login".to_string(),
                "from_broadcaster_user_name".to_string(),
                "to_broadcaster_user_id".to_string(),
                "to_broadcaster_user_login".to_string(),
                "to_broadcaster_user_name".to_string(),
                "viewers".to_string(),
            ],
        }]
    }

    fn spawn(&self) -> ProcessInit {
        Box::pin(SubscriberProcess::initializer(self.clone()))
    }
}

pub struct SubscriberProcess {
    client: HelixClient<'static, reqwest::Client>,
    token: UserToken,
    channel_id: UserId,
    socket: WSConnection,
    session_id: Option<String>,
    reconnect_url: Option<url::Url>,
}

impl SubscriberProcess {
    async fn initializer(component: SubscriberComponent) -> Result<Box<dyn Process + Send>> {
        let client: HelixClient<reqwest::Client> = HelixClient::default();
        let token = UserToken::from_token(&client, component.oauth.clone())
            .await
            .into_diagnostic()?;

        let channel_id = Self::get_user_id_for(&client, &token, &component.channel.clone()).await?;
        let socket = Self::connect().await?;

        Ok(Box::new(Self {
            client,
            token,
            channel_id,
            socket,
            session_id: None,
            reconnect_url: None,
        }))
    }

    pub async fn get_user_id_for<'a>(
        client: &HelixClient<'a, reqwest::Client>,
        token: &UserToken,
        name: &UserName,
    ) -> Result<UserId> {
        client
            .get_user_from_login(name, token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    async fn connect() -> Result<WSConnection> {
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

    async fn process_message(&mut self, msg: String) -> Result<Vec<Packet>> {
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

                self.client
                    .req_post(req, body, &self.token)
                    .await
                    .into_diagnostic()?;
            }
            Notification { metadata, payload } => {
                return self.process_notification(&metadata, &payload).await;
            }
            Revocation { .. } => (),
            Keepalive { .. } => (),
            _ => (),
        }

        Ok(vec![])
    }

    async fn process_notification<'a>(
        &mut self,
        _metadata: &NotificationMetadata<'a>,
        payload: &Event,
    ) -> Result<Vec<Packet>> {
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
                Ok(vec![Packet {
                    port: "raid".to_string(),
                    message,
                }])
            }
            _ => Ok(vec![]),
        }
    }
}

#[async_trait]
impl Process for SubscriberProcess {
    async fn run(&mut self, conn: &mut Connection) -> Result<()> {
        self.passive_run(conn).await
    }
}

#[async_trait]
impl PassiveProcess for SubscriberProcess {
    async fn handler(&mut self) -> Result<Vec<Packet>> {
        use tokio_tungstenite::tungstenite::*;

        match self.socket.next().await {
            Some(Ok(Message::Text(msg))) => {
                return match self.process_message(msg).await {
                    Ok(ps) => Ok(ps),
                    Err(err) => {
                        // catch and we will continue to process for next message.
                        eprintln!("message process error: {}", err);
                        Ok(vec![])
                    }
                };
            }
            Some(err @ Err(Error::ConnectionClosed)) => {
                // but if twitch says you should close connection, we want to along with that.
                err.into_diagnostic()
                    .wrap_err("Twitch connection was closed.")?;
            }
            None
            | Some(Err(Error::Protocol(error::ProtocolError::ResetWithoutClosingHandshake))) => {
                self.socket = Self::connect().await?;
            }
            _ => (),
        }
        Ok(vec![])
    }
}
