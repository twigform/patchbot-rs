use crate::{Context, Error};
use reqwest::header::USER_AGENT;

#[poise::command(prefix_command)]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let client = reqwest::Client::new();

    let response: serde_json::Value = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let cat_url = response[0]["url"].as_str().unwrap_or("no cat found :(");

    ctx.reply(cat_url).await?;

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pawng";
    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn consequence(
    ctx: Context<'_>,
    #[description = "User to punish."] user: serenity::model::user::User,
) -> Result<(), Error> {
    user.dm(
        &ctx.http(),
        serenity::builder::CreateMessage::new().content("Your consequence has been delivered."),
    )
    .await?;

    ctx.reply(format!(
        "Consequence initiated for {}.",
        user.display_name()
    ))
    .await?;
    Ok(())
}
