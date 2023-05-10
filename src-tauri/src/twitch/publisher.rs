use twitch_api::{
    helix::HelixClient,
    helix::{self, chat::AnnouncementColor},
    twitch_oauth2::{UserToken, AccessToken},
    types::{UserId, UserName},
};

use miette::{miette, IntoDiagnostic, Result};

use crate::{pipeline::{Connection, Property}, error::MrDamianError};

pub struct Publisher {
    pub client: HelixClient<'static, reqwest::Client>,
    pub oauth: AccessToken,
    pub token: Option<UserToken>,

    pub channel: UserName,
    pub channel_id: UserId,

    pub bot: UserName,
    pub bot_id: UserId,

    pub connection: Connection,
}

impl Publisher {
    pub fn new(bot: &str, channel: &str, oauth: &str) -> Self {
        let client: HelixClient<reqwest::Client> = HelixClient::default();

        Self {
            client,
            oauth: oauth.into(),
            token: None,
            channel: channel.into(),
            channel_id: "".into(),
            bot: bot.into(),
            bot_id: "".into(),
            connection: Connection::new("TwitchPublisher"),
        }
    }

    pub async fn setup(&mut self) -> Result<()> {
        let token = UserToken::from_token(&self.client, self.oauth.clone())
            .await
            .into_diagnostic()?;
        self.token = Some(token);

        self.channel_id = self.get_user_id_for(&self.channel.clone()).await?;
        self.bot_id = self.get_user_id_for(&self.bot.clone()).await?;

        Ok(())
    }

    async fn get_user_id_for(&mut self, name: &UserName) -> Result<UserId> {
        let token = self.token.as_ref().ok_or_else(MrDamianError::InvalidToken)?;
        self.client
            .get_user_from_login(name, token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    async fn send_shoutout(&mut self, to_broadcaster: &UserId) -> Result<()> {
        let token = self.token.as_ref().ok_or_else(MrDamianError::InvalidToken)?;
        let req = helix::chat::SendAShoutoutRequest::new(
            self.channel_id.clone(),
            to_broadcaster.clone(),
            self.bot_id.clone(),
        );

        self.client
            .req_post(req, Default::default(), token)
            .await
            .into_diagnostic()?;
        Ok(())
    }

    async fn send_notification(&mut self, message: &str) -> Result<()> {
        let token = self.token.as_ref().ok_or_else(MrDamianError::InvalidToken)?;
        self.client
            .send_chat_announcement(
                self.channel_id.as_str(),
                self.bot_id.as_str(),
                message,
                AnnouncementColor::Primary,
                token,
            )
            .await
            .into_diagnostic()?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let token = self.token.as_ref().ok_or_else(MrDamianError::InvalidToken)?;
            let packet = self.connection.receive()?;

            if packet.port != "message" {
                // drop all packets that are not from the message port.
                continue
            }
            let msg = packet.message;

            // TODO: allow users to customize the message via text formating component.
            let Some(Property::Text(flogin)) = msg.get("from_broadcaster_user_login") else {
                return Err(crate::error::MrDamianError::MessageKeyNotFound.into());
            };
            let Some(Property::Text(fid)) = msg.get("from_broadcaster_user_id") else {
                return Err(crate::error::MrDamianError::MessageKeyNotFound.into());
            };
            let Some(Property::I64(viewers)) = msg.get("viewers") else {
                return Err(crate::error::MrDamianError::MessageKeyNotFound.into());
            };
            let fid: UserId = fid.as_str().into();

            let user = self
                .client
                .get_user_from_id(&fid, token)
                .await
                .into_diagnostic()?
                .ok_or_else(|| miette!("No user found for channel {}.", flogin))?;

            let channel = self
                .client
                .get_channel_from_id(&fid, token)
                .await
                .into_diagnostic()?
                .ok_or_else(|| miette!("No channel info found for the user {}.", flogin))?;

            let message = format!(
                "{}さんから{}名のRAIDを頂きました！今日は「{}」を遊んでいたみたい",
                user.login, viewers, channel.game_name,
            );

            self.send_notification(&message).await?;
            self.send_shoutout(&fid).await?;
        }
    }
}
