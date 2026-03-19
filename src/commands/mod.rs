use crate::Error;
use once_cell::sync::Lazy;
use reqwest::header::USER_AGENT;
use tokio::sync::Mutex;

mod general;
mod gifs;
mod osu;
mod utility;

pub use general::{cat, consequence, ping};
pub use gifs::rgif;
pub use osu::{hgraph, lb, osu, osur};
pub use utility::{help, shutdown};

static OSU_TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub(super) async fn get_osu_token(client: &reqwest::Client) -> Result<String, Error> {
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

pub(super) fn format_num(n: u64) -> String {
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
