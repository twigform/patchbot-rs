use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use reqwest::header::USER_AGENT;
use std::fs;

#[poise::command(prefix_command)]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let client = reqwest::Client::new();

    let response: serde_json::Value = client
        .get("https://cataas.com/cat?json=1")
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let cat_url = response["url"].as_str().unwrap_or("no cat found :(");

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
pub async fn hi(ctx: Context<'_>) -> Result<(), Error> {
    let response = "hi";
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

const STATUS_FILE: &str = "src/status.txt";
const DEFAULT_STATUS: &str = "Running >help!";

pub fn load_status() -> String {
    fs::read_to_string(STATUS_FILE)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_STATUS.to_string())
}

fn save_status(status: &str) -> std::io::Result<()> {
    fs::write(STATUS_FILE, status)
}

#[poise::command(prefix_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "status to set"]
    #[rest]
    status: String,
) -> Result<(), Error> {
    ctx.serenity_context().set_presence(
        Some(serenity::ActivityData::custom(format!("{}", status))),
        serenity::OnlineStatus::Online,
    );

    save_status(&status)?;

    let response = format!("✔ Status set to '{}'!", status);
    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn name(
    ctx: Context<'_>,
    #[description = "server nickname to set"]
    #[rest]
    nickname: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Can only be used in a server :(")?;

    guild_id.edit_nickname(ctx.http(), Some(&nickname)).await?;

    let response = format!("✔ Server name set to '{}'!", nickname);
    ctx.reply(response).await?;
    Ok(())
}
