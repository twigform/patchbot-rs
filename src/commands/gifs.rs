use crate::{Context, Error};
use random_word::Lang;
use reqwest::header::USER_AGENT;

#[poise::command(prefix_command)]
pub async fn rgif(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let word = random_word::get(Lang::En);

    let client = reqwest::Client::new();
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

    ctx.reply(gif_url).await?;

    Ok(())
}
