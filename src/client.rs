use futures::StreamExt;
use twitch_api::{
    eventsub::{Event, EventsubWebsocketData, ReconnectPayload, WelcomePayload},
    helix::HelixClient,
    twitch_oauth2::UserToken,
    types::UserId,
};

use miette::{miette, IntoDiagnostic, Result, WrapErr};

pub struct Client {
    pub client: HelixClient<'static, reqwest::Client>,

    pub channel: String,
    pub token: UserToken,

    pub session_id: Option<String>,
    pub reconnect_url: Option<url::Url>,
}

type Connection = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

impl Client {
    pub async fn new(channel: &str, oauth: &str) -> Result<Self> {
        let client: HelixClient<reqwest::Client> = HelixClient::default();
        let token = UserToken::from_existing(&client, oauth.into(), None, None)
            .await
            .into_diagnostic()?;
        Ok(Self {
            client,
            channel: channel.to_string(),
            token,
            session_id: None,
            reconnect_url: None,
        })
    }

    pub async fn get_user_id(&mut self) -> Result<UserId> {
        self.client
            .get_user_from_login(&self.channel, &self.token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    pub async fn connect(&mut self) -> Result<Connection> {
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

    pub async fn run(&mut self) -> Result<()> {
        let mut socket = self.connect().await?;

        while let Some(msg) = socket.next().await {
            println!("running loop");
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Text(t)) => {
                    println!("Message: {}", &t);
                    use EventsubWebsocketData::*;
                    match Event::parse_websocket(t.as_str()).into_diagnostic()? {
                        Welcome { payload: WelcomePayload{ session }, .. } |
                        Reconnect { payload: ReconnectPayload{ session }, .. } => {
                            let user_id = self.get_user_id().await?;

                            self.session_id = Some(session.id.to_string());
                            if let Some(url) = session.reconnect_url {
                                self.reconnect_url = Some(url.parse().into_diagnostic()?);
                            }

                            let req = twitch_api::helix::eventsub::CreateEventSubSubscriptionRequest::default();
                            let body = twitch_api::helix::eventsub::CreateEventSubSubscriptionBody::new(
                                twitch_api::eventsub::channel::ChannelRaidV1::to_broadcaster_user_id(user_id),
                                twitch_api::eventsub::Transport::websocket(session.id.to_string()),
                            );

                            self.client.req_post(req, body, &self.token).await.into_diagnostic()?;

                            println!("Subscribed to raid event.");
                        },
                        Notification { payload, .. } => {
                            print!("Raid has come ===> {:?}", payload);
                        },
                        Revocation { .. } => (),
                        Keepalive { .. } => (),
                        _ => (),
                    }
                },
                Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => {
                    println!("Connection closed")
                },
                Err(tokio_tungstenite::tungstenite::Error::Protocol(
                    tokio_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    socket = self.connect().await?;
                },
                _ => (),
            }
        }

        Ok(())
    }
}
