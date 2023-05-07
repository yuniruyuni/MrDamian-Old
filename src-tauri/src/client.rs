use futures::StreamExt;
use twitch_api::{
    eventsub::{
        Event, EventsubWebsocketData, Message, NotificationMetadata, Payload, ReconnectPayload,
        WelcomePayload,
    },
    helix::HelixClient,
    helix::{self, chat::AnnouncementColor},
    twitch_oauth2::UserToken,
    types::{UserId, UserName},
};

use miette::{miette, IntoDiagnostic, Result, WrapErr};

pub struct Client {
    pub client: HelixClient<'static, reqwest::Client>,

    pub channel: UserName,
    pub channel_id: UserId,

    pub token: UserToken,

    pub bot: UserName,
    pub bot_id: UserId,

    pub session_id: Option<String>,
    pub reconnect_url: Option<url::Url>,
}

type Connection =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

impl Client {
    pub async fn new(bot: &str, channel: &str, oauth: &str) -> Result<Self> {
        let client: HelixClient<reqwest::Client> = HelixClient::default();
        let token = UserToken::from_existing(&client, oauth.into(), None, None)
            .await
            .into_diagnostic()?;

        Ok(Self {
            client,
            channel: channel.into(),
            bot: bot.into(),
            token,
            session_id: None,
            reconnect_url: None,
            channel_id: "".into(),
            bot_id: "".into(),
        })
    }

    pub async fn get_user_id_for(&mut self, name: &UserName) -> Result<UserId> {
        self.client
            .get_user_from_login(name, &self.token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    pub async fn send_shoutout(&mut self, to_broadcaster: &UserId) -> Result<()> {
        let req = helix::chat::SendAShoutoutRequest::new(
            self.channel_id.clone(),
            to_broadcaster.clone(),
            self.bot_id.clone(),
        );

        self.client
            .req_post(req, Default::default(), &self.token)
            .await
            .into_diagnostic()?;
        Ok(())
    }

    async fn send_notification(&mut self, message: &str) -> Result<()> {
        self.client
            .send_chat_announcement(
                self.channel_id.as_str(),
                self.bot_id.as_str(),
                message,
                AnnouncementColor::Primary,
                &self.token,
            )
            .await
            .into_diagnostic()?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut socket = self.connect().await?;
        loop {
            use tokio_tungstenite::tungstenite::*;
            match socket.next().await {
                Some(Ok(Message::Text(msg))) => {
                    if let Err(err) = self.process_message(msg).await {
                        eprintln!("message process error: {}", err);
                    }
                },
                Some(err @ Err(Error::ConnectionClosed)) => {
                    // but if twitch says you should close connection, we want to along with that.
                    err.into_diagnostic().wrap_err("Twitch connection was closed.")?;
                },
                None | Some(Err(tokio_tungstenite::tungstenite::Error::Protocol(
                    tokio_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                ))) => {
                    socket = self.connect().await?;
                },
                _ => (),
            }
        }
    }

    async fn connect(&mut self) -> Result<Connection> {
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

                self.client
                    .req_post(req, body, &self.token)
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
                message: Message::Notification(msg),
                ..
            }) => {
                let flogin = msg.from_broadcaster_user_login.clone();

                let user = self
                    .client
                    .get_user_from_id(&msg.from_broadcaster_user_id, &self.token)
                    .await
                    .into_diagnostic()?
                    .ok_or_else(|| miette!("No user found for channel {}.", flogin))?;

                let channel = self
                    .client
                    .get_channel_from_id(&msg.from_broadcaster_user_id, &self.token)
                    .await
                    .into_diagnostic()?
                    .ok_or_else(|| miette!("No channel info found for the user {}.", flogin))?;

                let message = format!(
                    "{}さんから{}名のRAIDを頂きました！今日は「{}」を遊んでいたみたい",
                    user.login, msg.viewers, channel.game_name,
                );
                self.send_notification(&message).await?;
                self.send_shoutout(&msg.from_broadcaster_user_id).await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
