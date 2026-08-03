use crate::{Context, Error};
use reqwest::header::USER_AGENT;

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
    const MAX_RETRIES: u8 = 5;
    let mut slug: Option<String> = None;

    for _ in 0..MAX_RETRIES {
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

        if let Some(found) = response["data"]["data"][0]["slug"].as_str() {
            slug = Some(found.to_string());
            break;
        }
    }

    let gif_url = match slug {
        Some(s) => format!("https://klipy.com/gifs/{}", s),
        None => "no gif found :(".to_string(),
    };

    ctx.reply(gif_url).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn rgifword(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let client = reqwest::Client::new();
    const MAX_RETRIES: u8 = 5;
    let mut slug: Option<String> = None;
    let mut last_word = String::new();

    for _ in 0..MAX_RETRIES {
        let word = get_rand_word(&client).await?;
        last_word = word.clone();

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

        if let Some(found) = response["data"]["data"][0]["slug"].as_str() {
            slug = Some(found.to_string());
            break;
        }
    }

    let fresponse = match slug {
        Some(s) => {
            let gif_url = format!("https://klipy.com/gifs/{}", s);
            format!(
                "[Source link]({}) \nWord used in search: '{}'",
                gif_url, last_word
            )
        }
        None => "no gif found :(".to_string(),
    };

    ctx.reply(fresponse).await?;
    Ok(())
}
