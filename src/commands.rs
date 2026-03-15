use crate::{Context, Error};
use once_cell::sync::Lazy;
use poise::builtins::HelpConfiguration;
use reqwest::header::USER_AGENT;
use serenity::builder::CreateEmbed;
use tokio::sync::Mutex;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pawng";
    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(prefix_command, owners_only, category = "Utility")]
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

    let slug = response["data"]["data"][0]["slug"]
        .as_str()
        .unwrap_or("no gif found :(");

    let gif_url = format!("https://klipy.com/gifs/{}", slug);

    ctx.reply(gif_url).await?;

    Ok(())
}

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

#[poise::command(prefix_command, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Get details for a specific command"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }
    let extra_text_at_bottom = "\
Run `>help command` for info on a specific command.";

    let config = HelpConfiguration {
        extra_text_at_bottom,
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

static OSU_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

async fn get_osu_token(client: &reqwest::Client) -> Result<String, Error> {
    let mut token = OSU_TOKEN.lock().await;
    if let Some(t) = token.as_ref() {
        return Ok(t.clone());
    }

    let res: serde_json::Value = client
        .post("https://osu.ppy.sh/oauth/token")
        .header(USER_AGENT, "patchbot_discord")
        .json(&serde_json::json!({
            "client_id": std::env::var("OSU_CLIENT_ID").expect("missing OSU_CLIENT_ID! please make a .env file in the root of this project and add OSU_CLIENT_ID=OSU_CLIENT_ID HERE to it!"),
            "client_secret": std::env::var("OSU_CLIENT_SECRET").expect("missing OSU_CLIENT_SECRET! please make a .env file in the root of this project and add OSU_CLIENT_SECRET=OSU_CLIENT_SECRET HERE to it!"),
            "grant_type": "client_credentials",
            "scope": "public"
        }))
        .send()
        .await?
        .json()
        .await?;

    let access_token = res["access_token"]
        .as_str()
        .ok_or("failed to get osu token")?
        .to_string();

    *token = Some(access_token.clone());
    Ok(access_token)
}

fn format_num(n: u64) -> String {
    let s = n.to_string();
    let mut formatted = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }
    formatted.chars().rev().collect()
}

#[poise::command(prefix_command)]
pub async fn osu(
    ctx: Context<'_>,
    #[description = "osu! user to grab"] o_user: String,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let client = reqwest::Client::new();

    let token = get_osu_token(&client).await?;

    let response: serde_json::Value = client
        .get(format!("https://osu.ppy.sh/api/v2/users/{}/osu", o_user))
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let username = response["username"].as_str().unwrap_or("no user found :(");
    let uid = response["id"].as_u64().unwrap_or(0);

    if uid == 0 {
        ctx.send(poise::CreateReply::default().content("osu! user not found... :("))
            .await?;
        return Ok(());
    }

    let rank = response["statistics"]["global_rank"]
        .as_u64()
        .map(|r| format_num(r))
        .unwrap_or_else(|| "unranked".to_string());
    let is_online = response["is_online"].as_bool().unwrap_or(false);
    let title = format!("<:osu:1482134509729349812> osu! user: {}", username);
    let pfp = format!("https://a.ppy.sh/{}", uid);
    let url = format!("https://osu.ppy.sh/users/{}", uid);
    let online_str = if is_online {
        "<a:online:1482134508743426209> Online"
    } else {
        "<a:offline:1482135749985046651> Offline"
    };

    let recent: serde_json::Value = client
        .get(format!(
            "https://osu.ppy.sh/api/v2/users/{}/scores/recent?limit=1&include_fails=1",
            uid
        ))
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let last_played = recent[0]["beatmapset"]["title"]
        .as_str()
        .unwrap_or("nothing recent :(");

    let beatmap_url = recent[0]["beatmap"]["url"].as_str().unwrap_or("");

    let last_played_str = if beatmap_url.is_empty() {
        last_played.to_string()
    } else {
        format!("[{}]({})", last_played, beatmap_url)
    };

    let embed = CreateEmbed::new()
        .title(&title)
        .url(&url)
        .field("Rank", format!("#{}", &rank), false)
        .field("Status", online_str, false)
        .field("Last played:", &last_played_str, false)
        .color(0xFF66AA)
        .thumbnail(&pfp)
        .timestamp(serenity::model::Timestamp::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
