use futures::StreamExt;
use twitch_api::{
    types::UserId,
    eventsub::{Event, EventsubWebsocketData, WelcomePayload, ReconnectPayload},
    helix::{
        HelixClient,
    },
    twitch_oauth2::UserToken,
};

use miette::{IntoDiagnostic, Result, WrapErr};

pub struct Client {
    pub client: HelixClient<'static, reqwest::Client>,

    pub user_id: UserId,
    pub token: UserToken,

    pub session_id: Option<String>,
    pub reconnect_url: Option<url::Url>,
}

impl Client {
    pub async fn run(&mut self) -> Result<()> {
        let config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default();

        let (mut socket, _) = tokio_tungstenite::connect_async_with_config(
            twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            Some(config),
        )
        .await
        .into_diagnostic()
        .context("Cannot connect twitch event server host")?;

        while let Some(msg) = socket.next().await {
            println!("running loop");
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Text(t)) => {
                    println!("Message: {}", &t);
                    use EventsubWebsocketData::*;
                    match Event::parse_websocket(t.as_str()).into_diagnostic()? {
                        Welcome { payload: WelcomePayload{ session }, .. } |
                        Reconnect { payload: ReconnectPayload{ session }, .. } => {
                            self.session_id = Some(session.id.to_string());
                            if let Some(url) = session.reconnect_url {
                                self.reconnect_url = Some(url.parse().into_diagnostic()?);
                            }

                            let req = twitch_api::helix::eventsub::CreateEventSubSubscriptionRequest::default();
                            let body = twitch_api::helix::eventsub::CreateEventSubSubscriptionBody::new(
                                twitch_api::eventsub::channel::ChannelRaidV1::to_broadcaster_user_id(self.user_id.clone()),
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
                    let (s, _) = tokio_tungstenite::connect_async_with_config(
                            twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
                            Some(config),
                        )
                        .await
                        .into_diagnostic()
                        .context("Cannot connect twitch event server host")?;
                    socket = s;
                },
                _ => (),
            }
        }

        Ok(())
    }
}