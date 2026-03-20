use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use reqwest::header::USER_AGENT;

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

// #[poise::command(prefix_command)]
// pub async fn status(
//     ctx: Context<'_>,
//     #[description = "status to set"] o_user: String,
// ) -> Result<(), Error> {
//     ctx.set_presence(
//         Some(serenity::ActivityData::custom("Running >help!")),
//         serenity::OnlineStatus::Online,
//     );
//     let response = "pawng";
//     ctx.reply(response).await?;
//     Ok(())
// }
