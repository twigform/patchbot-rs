use crate::{Context, Error};
use reqwest::header::USER_AGENT;

// add a loop for if no gif found later

async fn get_rand_word(client: &reqwest::Client) -> Result<String, Error> {
    let words: Vec<String> = client
        .get("https://random-word-api.herokuapp.com/word")
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    Ok(words
        .into_iter()
        .next()
        .unwrap_or_else(|| "word".to_string()))
}

#[poise::command(prefix_command)]
pub async fn rgif(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let client = reqwest::Client::new();
    let word = get_rand_word(&client).await?;

    let url = format!(
        "https://api.klipy.com/api/v1/{}/gifs/search?q={}&per_page=1&content_filter=off",
        std::env::var("KLIPY_API").expect("missing klipy key! please make a .env file in the root of this project and add KLIPY_API=KLIPY API KEY HERE to it!"),
        word
    );

    let response: serde_json::Value = client
        .get(&url)
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let slug = response["data"]["data"][0]["slug"]
        .as_str()
        .unwrap_or("no gif found :(");

    let gif_url = format!("https://klipy.com/gifs/{}", slug);

    ctx.reply(gif_url).await?;

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn rgifword(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let client = reqwest::Client::new();
    let word = get_rand_word(&client).await?;

    let url = format!(
        "https://api.klipy.com/api/v1/{}/gifs/search?q={}/&per_page=1",
        std::env::var("KLIPY_API").expect("missing klipy key! please make a .env file in the root of this project and add KLIPY_API=KLIPY API KEY HERE to it!"),
        word
    );

    let response: serde_json::Value = client
        .get(&url)
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let slug = response["data"]["data"][0]["slug"]
        .as_str()
        .unwrap_or("no gif found :(");

    let gif_url = format!("https://klipy.com/gifs/{}", slug);

    let fresponse = format!(
        "[Source link]({}) \nWord used in search: '{}'",
        gif_url, word
    );

    ctx.reply(fresponse).await?;

    Ok(())
}
