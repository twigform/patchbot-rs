use crate::{Context, Error};
use reqwest::header::USER_AGENT;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pawng";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("ok bye :(").await?;
    std::process::exit(0);
}

async fn get_rand_word() -> Result<String, Error> {
    let client = reqwest::Client::new();

    let words: Vec<String> = client
        .get("https://random-word-api.herokuapp.com/word")
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let response = words
        .first()
        .cloned()
        .unwrap_or("no word found".to_string());

    Ok(response)
}

#[poise::command(prefix_command)]
pub async fn rgif(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let word = get_rand_word().await?;

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.klipy.com/api/v1/{}/gifs/search?q={}/&per_page=1",
        std::env::var("KLIPY_API").expect("missing klipy key! please make a .env file in the root of this project and add KLIPY_API=KLIPY API KEY HERE to it!"),
        word
    );

    let response: serde_json::Value = client
        .get(&url)
        .header(USER_AGENT, "patchbot_discord") // maybe make this a var later
        .send()
        .await?
        .json()
        .await?;

    let gif_url = response["data"]["data"][0]["file"]["md"]["gif"]["url"]
        .as_str()
        .unwrap_or("no gif found :(")
        .to_string();

    ctx.say(gif_url).await?;

    Ok(())
}
