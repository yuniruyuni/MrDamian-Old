mod client;

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

use client::Client;

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

    let token = UserToken::from_existing(&client, oauth.into(), None, None)
            .await
            .into_diagnostic()?;

    let user_id = client
        .get_user_from_login(&channel, &token)
        .await
        .into_diagnostic()?
        .ok_or_else(|| miette!("No user found for channel {channel}."))?
        .id;

    println!("UserID: {user_id}");

    let mut wsclient = Client{
        client,
        user_id,
        token: token,

        session_id: None,
        reconnect_url: None,
    };

    wsclient.run().await?;

    Ok(())
}
