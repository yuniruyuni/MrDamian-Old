use futures::StreamExt;
use std::{env, sync::Arc};
use tokio::sync::RwLock;
use twitch_api::{
    types::UserId,
    eventsub::{Event, EventsubWebsocketData, WelcomePayload, ReconnectPayload},
    helix::{
        HelixClient,
    },
    twitch_oauth2::UserToken,
};

use miette::{miette, IntoDiagnostic, Result, WrapErr};

struct Client {
    client: HelixClient<'static, reqwest::Client>,

    user_id: UserId,
    token: Arc<RwLock<UserToken>>,

    session_id: Option<String>,
    reconnect_url: Option<url::Url>,
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

                            self.client.req_post(req, body, &*self.token.read().await).await.into_diagnostic()?;

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

#[tokio::main]
async fn main() -> Result<()> {
    let channel = env::var("TWITCH_CHANNEL")
        .into_diagnostic()
        .context("TWITCH_CHANNEL must be set.")?;
    // TODO: rename envvar name.
    let oauth = env::var("TWITCH_OAUTH")
        .into_diagnostic()
        .context("TWITCH_OAUTH must be set.")?;

    let client: HelixClient<reqwest::Client> = HelixClient::default();

    let token = Arc::new(RwLock::new(
        UserToken::from_existing(&client, oauth.into(), None, None)
            .await
            .into_diagnostic()?,
    ));

    let user_id = client
        .get_user_from_login(&channel, &*token.read().await)
        .await
        .into_diagnostic()?
        .ok_or_else(|| miette!("No user found for channel {channel}."))?
        .id;

    println!("UserID: {user_id}");

    let mut wsclient = Client{
        client,
        user_id,
        token,

        session_id: None,
        reconnect_url: None,
    };

    wsclient.run().await?;

    Ok(())
}
