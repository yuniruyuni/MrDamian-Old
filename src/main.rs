use std::{env, sync::Arc};
use tokio::sync::RwLock;
use twitch_api::{helix::HelixClient, twitch_oauth2::UserToken};

use miette::{miette, IntoDiagnostic, WrapErr, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let channel = env::var("TWITCH_CHANNEL")
        .into_diagnostic()
        .wrap_err("TWITCH_CHANNEL must be set.")?;
    // TODO: rename envvar name.
    let oauth = env::var("TWITCH_OAUTH")
        .into_diagnostic()
        .wrap_err("TWITCH_OAUTH must be set.")?;

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

    Ok(())
}
