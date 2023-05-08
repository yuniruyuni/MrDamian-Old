use twitch_api::{
    helix::HelixClient,
    helix::{self, chat::AnnouncementColor},
    twitch_oauth2::UserToken,
    types::{UserId, UserName},
};

use std::sync::mpsc::Receiver;

use miette::{miette, IntoDiagnostic, Result};

use crate::pipeline::{Message, Property};

pub struct Publisher {
    pub receiver: Receiver<Message>,

    pub client: HelixClient<'static, reqwest::Client>,
    pub token: UserToken,

    pub channel: UserName,
    pub channel_id: UserId,

    pub bot: UserName,
    pub bot_id: UserId,
}

impl Publisher {
    pub async fn new(
        receiver: Receiver<Message>,
        bot: &str,
        channel: &str,
        oauth: &str,
    ) -> Result<Self> {
        let client: HelixClient<reqwest::Client> = HelixClient::default();
        let token = UserToken::from_existing(&client, oauth.into(), None, None)
            .await
            .into_diagnostic()?;

        let mut res = Self {
            receiver,
            client,
            token,
            channel: channel.into(),
            channel_id: "".into(),
            bot: bot.into(),
            bot_id: "".into(),
        };

        res.channel_id = res.get_user_id_for(&res.channel.clone()).await?;
        res.bot_id = res.get_user_id_for(&res.bot.clone()).await?;

        Ok(res)
    }

    async fn get_user_id_for(&mut self, name: &UserName) -> Result<UserId> {
        self.client
            .get_user_from_login(name, &self.token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    async fn send_shoutout(&mut self, to_broadcaster: &UserId) -> Result<()> {
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
        loop {
            let msg = self.receiver.recv().into_diagnostic()?;

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
                .get_user_from_id(&fid, &self.token)
                .await
                .into_diagnostic()?
                .ok_or_else(|| miette!("No user found for channel {}.", flogin))?;

            let channel = self
                .client
                .get_channel_from_id(&fid, &self.token)
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
