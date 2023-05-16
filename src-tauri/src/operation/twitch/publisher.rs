use async_trait::async_trait;

use twitch_api::{
    helix::HelixClient,
    helix::{self, chat::AnnouncementColor},
    twitch_oauth2::{AccessToken, UserToken},
    types::{UserId, UserName},
};

use miette::{miette, IntoDiagnostic, Result};

use crate::{
    model::error::MrDamianError,
    model::{InputPort, InputPortID, OutputPort},
    operation::pipeline::{
        Component, Connection, Constructor, DefaultProcess, Packet, Process, ProcessInit, Property,
    },
};

#[derive(Debug, Clone)]
pub struct PublisherComponent {
    id: String,
    oauth: AccessToken,
    channel: UserName,
    bot: UserName,
}

impl PublisherComponent {
    pub fn constructor() -> Constructor {
        Constructor {
            kind: "TwitchPublisher",
            label: "Twitch Publisher",
            gen: Box::new(
                |id: &str, config: &crate::config::Config| -> Box<dyn Component + Send> {
                    Box::new(PublisherComponent::new(
                        id,
                        &config.bot,
                        &config.channel,
                        &config.token,
                    ))
                },
            ),
        }
    }

    pub fn new(id: &str, bot: &str, channel: &str, oauth: &str) -> Self {
        Self {
            id: id.to_string(),
            oauth: oauth.into(),
            channel: channel.into(),
            bot: bot.into(),
        }
    }
}

impl Component for PublisherComponent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn kind(&self) -> &'static str {
        "TwitchPublisher"
    }

    fn label(&self) -> &'static str {
        "Twitch Publisher"
    }

    fn inputs(&self) -> Vec<InputPort> {
        vec![InputPort {
            id: InputPortID {
                parent: self.id.to_string(),
                name: "message".to_string(),
            },
            property_names: vec![
                "from_broadcaster_user_login".to_string(),
                "from_broadcaster_user_id".to_string(),
                "viewers".to_string(),
            ],
        }]
    }

    fn outputs(&self) -> Vec<OutputPort> {
        vec![]
    }

    fn spawn(&self) -> ProcessInit {
        Box::pin(PublisherProcess::initializer(self.clone()))
    }
}

pub struct PublisherProcess {
    client: HelixClient<'static, reqwest::Client>,
    token: UserToken,
    channel_id: UserId,
    bot_id: UserId,
}

impl PublisherProcess {
    pub async fn initializer(component: PublisherComponent) -> Result<Box<dyn Process + Send>> {
        let client: HelixClient<reqwest::Client> = HelixClient::default();

        let token = UserToken::from_token(&client, component.oauth.clone())
            .await
            .into_diagnostic()?;

        let channel_id = Self::get_user_id_for(&client, &token, &component.channel.clone()).await?;
        let bot_id = Self::get_user_id_for(&client, &token, &component.bot.clone()).await?;

        Ok(Box::new(Self {
            client,
            token,
            channel_id,
            bot_id,
        }))
    }

    async fn get_user_id_for<'a>(
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
}

#[async_trait]
impl Process for PublisherProcess {
    async fn run(&mut self, conn: &mut Connection) -> Result<()> {
        self.default_run(conn).await
    }
}

#[async_trait]
impl DefaultProcess for PublisherProcess {
    async fn handler(&mut self, packet: Packet) -> Result<Vec<Packet>> {
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
        Ok(vec![])
    }
}
