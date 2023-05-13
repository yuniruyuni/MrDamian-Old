use twitch_api::{
    helix::HelixClient,
    helix::{self, chat::AnnouncementColor},
    twitch_oauth2::{AccessToken, UserToken},
    types::{UserId, UserName},
};

use async_trait::async_trait;
use miette::{miette, IntoDiagnostic, Result};

use crate::{
    error::MrDamianError,
    pipeline::{Component, Connection, DefaultComponent, Packet, Property},
};

pub struct Publisher {
    client: HelixClient<'static, reqwest::Client>,
    oauth: AccessToken,
    token: Option<UserToken>,

    channel: UserName,
    channel_id: UserId,

    bot: UserName,
    bot_id: UserId,

    conn: Connection,
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
            conn: Connection::new(),
        }
    }

    async fn get_user_id_for(&mut self, name: &UserName) -> Result<UserId> {
        let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;
        self.client
            .get_user_from_login(name, token)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette!("No user found for channel {channel}."))
            .map(|user| user.id)
    }

    async fn send_shoutout(&mut self, to_broadcaster: &UserId) -> Result<()> {
        let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;
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
        let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;
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
}

#[async_trait]
impl Component for Publisher {
    fn name(&self) -> String {
        "TwitchPublisher".to_string()
    }

    fn connection(&mut self) -> &mut Connection {
        &mut self.conn
    }

    async fn run(&mut self) -> Result<()> {
        self.default_run().await
    }
}

#[async_trait]
impl DefaultComponent for Publisher {
    async fn setup(&mut self) -> Result<()> {
        let token = UserToken::from_token(&self.client, self.oauth.clone())
            .await
            .into_diagnostic()?;
        self.token = Some(token);

        self.channel_id = self.get_user_id_for(&self.channel.clone()).await?;
        self.bot_id = self.get_user_id_for(&self.bot.clone()).await?;

        Ok(())
    }

    async fn handler(&mut self, packet: Packet) -> Result<Vec<Packet>> {
        let token = self.token.as_ref().ok_or(MrDamianError::InvalidToken)?;

        if packet.port != "message" {
            // drop all packets that are not from the message port.
            return Ok(vec![]);
        }

        let msg = packet.message;

        // TODO: allow users to customize the message via text formating component.
        let Some(Property::Text(flogin)) = msg.get("from_broadcaster_user_login") else {
            return Err(MrDamianError::MessageKeyNotFound.into());
        };
        let Some(Property::Text(fid)) = msg.get("from_broadcaster_user_id") else {
            return Err(MrDamianError::MessageKeyNotFound.into());
        };
        let Some(Property::I64(viewers)) = msg.get("viewers") else {
            return Err(MrDamianError::MessageKeyNotFound.into());
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
        Ok(vec![])
    }
}

use crate::pipeline::Constructor;
use crate::protocol::{InputPort, OutputPort};

#[derive(Debug, Default, Clone)]
pub struct PublisherFactory {}

impl Constructor for PublisherFactory {
    fn component_type(&self) -> String {
        "TwitchPublisher".to_string()
    }
    fn construct(&self, config: &crate::config::Config) -> Box<dyn Component + Send> {
        Box::new(Publisher::new(&config.bot, &config.channel, &config.token))
    }
    fn label(&self) -> String {
        "Twitch Publisher".to_string()
    }
    fn inputs(&self) -> Vec<InputPort> {
        vec![InputPort {
            name: "raid".to_string(),
        }]
    }
    fn outputs(&self) -> Vec<OutputPort> {
        vec![]
    }
}
