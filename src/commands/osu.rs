use super::{format_num, get_osu_token};
use crate::{Context, Error};
use quickchart_rs::QuickchartClient;
use reqwest::header::USER_AGENT;
use serenity::builder::CreateEmbed;

#[poise::command(prefix_command, category = "osu!")]
pub async fn osu(
    ctx: Context<'_>,
    #[description = "osu! user to grab"] o_user: String,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;
    let client = reqwest::Client::new();
    let token = get_osu_token(&client).await?;

    let response: serde_json::Value = client
        .get(format!("https://osu.ppy.sh/api/v2/users/{}", o_user))
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

    let playmode = response["playmode"].as_str().unwrap_or("osu");

    let response: serde_json::Value = client
        .get(format!(
            "https://osu.ppy.sh/api/v2/users/{}/{}",
            uid, playmode
        ))
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let rank = response["statistics"]["global_rank"]
        .as_u64()
        .map(format_num)
        .unwrap_or_else(|| "unranked".to_string());
    let is_online = response["is_online"].as_bool().unwrap_or(false);
    let title = format!("<:osu:1482134509729349812> osu! user: {}", username);
    let pfp = format!("https://a.ppy.sh/{}", uid);
    let url = format!("https://osu.ppy.sh/users/{}/{}", uid, playmode);
    let pp = response["statistics"]["pp"]
        .as_f64()
        .map(|r| format!("{:.2}", r))
        .unwrap_or_else(|| "unranked".to_string());

    let online_str = if is_online {
        "<a:online:1482134508743426209> Online".to_string()
    } else {
        match response["last_visit"].as_str() {
            Some(last_visit) => match chrono::DateTime::parse_from_rfc3339(last_visit) {
                Ok(dt) => format!(
                    "<a:offline:1482135749985046651> Offline • last seen <t:{}:R>",
                    dt.timestamp()
                ),
                Err(_) => "<a:offline:1482135749985046651> Offline".to_string(),
            },
            None => "<a:offline:1482135749985046651> Offline".to_string(),
        }
    };

    let recent: serde_json::Value = client
        .get(format!(
            "https://osu.ppy.sh/api/v2/users/{}/scores/recent?limit=1&include_fails=1&mode={}",
            uid, playmode
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
        .field("Rank", format!("#{}", &rank), true)
        .field("PP", &pp, true)
        .field("Status", online_str, false)
        .field("Last played:", &last_played_str, false)
        .color(0xFF66AA)
        .thumbnail(&pfp);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(prefix_command, category = "osu!")]
pub async fn osur(
    ctx: Context<'_>,
    #[description = "osu! user to grab recent map for"] o_user: String,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let client = reqwest::Client::new();
    let token = get_osu_token(&client).await?;

    let user_response: serde_json::Value = client
        .get(format!("https://osu.ppy.sh/api/v2/users/{}/osu", o_user))
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let uid = user_response["id"].as_u64().unwrap_or(0);
    let username = user_response["username"].as_str().unwrap_or("").to_string();

    if uid == 0 {
        ctx.send(poise::CreateReply::default().content("osu! user not found... :("))
            .await?;
        return Ok(());
    }

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

    let score = &recent[0];

    if score.is_null() {
        ctx.send(poise::CreateReply::default().content("no recent scores for this user... :("))
            .await?;
        return Ok(());
    }

    let m_title = score["beatmapset"]["title"].as_str().unwrap_or("Unknown");
    let m_artist = score["beatmapset"]["artist"].as_str().unwrap_or("Unknown");
    let m_difficulty = score["beatmap"]["version"].as_str().unwrap_or("Unknown");
    let bm_url = score["beatmap"]["url"].as_str().unwrap_or("");
    let c_url = score["beatmapset"]["covers"]["cover"]
        .as_str()
        .unwrap_or("");

    fn grade_2_emoji(rank: &str) -> String {
        let e_id = match rank {
            "A" => "1483851816675315854",
            "B" => "1483851819867443240",
            "C" => "1483851821620531220",
            "D" => "1483851822744731798",
            "F" => "1483864786360991906",
            "S" => "1483851823570878567",
            "SH" => "1483851824392966224",
            "SS" => "1483851825349267477",
            "SSH" => "1483851826452234373",
            _ => return rank.to_string(),
        };
        format!("<:ranking{}:{}>", rank, e_id)
    }

    let rank = grade_2_emoji(score["rank"].as_str().unwrap_or("?"));
    let pp = score["pp"].as_f64();
    let accuracy = score["accuracy"].as_f64().unwrap_or(0.0) * 100.0;
    let max_combo = score["max_combo"].as_u64().unwrap_or(0);
    let mods: Vec<String> = score["mods"]
        .as_array()
        .map(|m| {
            m.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let c_300 = score["statistics"]["count_300"].as_u64().unwrap_or(0);
    let c_100 = score["statistics"]["count_100"].as_u64().unwrap_or(0);
    let c_50 = score["statistics"]["count_50"].as_u64().unwrap_or(0);
    let c_miss = score["statistics"]["count_miss"].as_u64().unwrap_or(0);

    let stars = score["beatmap"]["difficulty_rating"]
        .as_f64()
        .unwrap_or(0.0);

    let bm_id = score["beatmap"]["id"].as_u64().unwrap_or(0);

    let bm_data: serde_json::Value = client
        .get(format!("https://osu.ppy.sh/api/v2/beatmaps/{}", bm_id))
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let bm_max_combo = bm_data["max_combo"].as_u64().unwrap_or(0);

    let mods_str = if mods.is_empty() {
        "None".to_string()
    } else {
        mods.join(", ")
    };

    let pp_str = match pp {
        Some(p) => format!("{:.2}pp", p),
        None => "N/A (failed or unranked)".to_string(),
    };

    let combo_str = format!("{}x / {}x", max_combo, bm_max_combo);

    let title = format!("<:osu:1482134509729349812> {}'s recent score", username);

    let m_str = format!(
        "[{} - {} [{}]]({})",
        m_artist, m_title, m_difficulty, bm_url
    );

    let embed = CreateEmbed::new()
        .title(&title)
        .url(format!("https://osu.ppy.sh/users/{}", uid))
        .description(&m_str)
        .field("Grade", rank, true)
        .field("PP", pp_str, true)
        .field("Accuracy", format!("{:.2}%", accuracy), true)
        .field("Combo", combo_str, true)
        .field("Stars", format!("{:.2} <:star:1482800848617738311>", stars), true)
        .field("Mods", mods_str, true)
        .field(
            "Hits",
            format!(
                "{} <:hit300t:1482804510819877078> • {} <:hit100t:1482804499759239169> • {} <:hit50:1482799044379017216> • {} <:hit0:1482799042982580315>",
                c_300, c_100, c_50, c_miss
            ),
            true,
        )
        .color(0xFF66AA)
        .thumbnail(format!("https://a.ppy.sh/{}", uid))
        .image(c_url);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(prefix_command, category = "osu!")]
pub async fn lb(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let client = reqwest::Client::new();

    let token = get_osu_token(&client).await?;

    let response: serde_json::Value = client
        .get("https://osu.ppy.sh/api/v2/rankings/osu/global?&filter=all")
        .header(USER_AGENT, "patchbot_discord")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let ranking = response["ranking"]
        .as_array()
        .ok_or("Missing ranking array")?;

    let mut embed = CreateEmbed::new()
        .title("<:osu:1482134509729349812> osu! global leaderboard")
        .url("https://osu.ppy.sh/rankings/osu/global")
        .color(0xFF66AA);

    for entry in ranking.iter().take(10) {
        let rank = entry["global_rank"].as_u64().unwrap_or(0);
        let pp = entry["pp"].as_f64().unwrap_or(0.0);
        let accuracy = entry["hit_accuracy"].as_f64().unwrap_or(0.0);
        let username = entry["user"]["username"].as_str().unwrap_or("Unknown");
        let country = entry["user"]["country"]["code"].as_str().unwrap_or("??");
        let uid = entry["user"]["id"].as_u64().unwrap_or(0);

        embed = embed.field(
            format!("#{rank}: {username} :flag_{}:", country.to_lowercase()),
            format!(
                "[Profile](https://osu.ppy.sh/users/{uid}) • **{pp:.0}pp** • {accuracy:.2}% acc"
            ),
            false,
        );
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(prefix_command, category = "osu!")]
pub async fn hgraph(
    ctx: Context<'_>,
    #[description = "osu! user(s) to grab ranked history for"]
    #[rest]
    users: String,
) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let o_users: Vec<&str> = users.split_whitespace().take(5).collect();

    if o_users.is_empty() {
        ctx.send(poise::CreateReply::default().content("please provide at least one user :("))
            .await?;
        return Ok(());
    }

    struct UserData {
        username: String,
        uid: u64,
        rank_history: Vec<i64>,
    }

    let mut all_users: Vec<UserData> = Vec::new();

    let client = reqwest::Client::new();
    let token = get_osu_token(&client).await?;

    let colors = ["#ff66aa", "#66aaff", "#66ffaa", "#ffaa66", "#aa66ff"];

    let c_emojis = [
        "<:circlePink:1484583245348737217>",
        "<:circleBlue:1484583241255092294>",
        "<:circleGreen:1484583243130081432>",
        "<:circleOrange:1484583244132515900>",
        "<:circlePurple:1484583246791573684>",
    ];

    for o_user in o_users {
        let response: serde_json::Value = client
            .get(format!("https://osu.ppy.sh/api/v2/users/{}/osu", o_user))
            .header(USER_AGENT, "patchbot_discord")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        let username = response["username"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let uid = response["id"].as_u64().unwrap_or(0);

        if uid == 0 {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("osu! user {} not found... :(", o_user)),
            )
            .await?;
            return Ok(());
        }

        let rank_history: Vec<i64> = response["rank_history"]["data"]
            .as_array()
            .map(|arr| {
                let all: Vec<i64> = arr
                    .iter()
                    .filter_map(|v| v.as_i64())
                    .filter(|r| *r > 0)
                    .collect();
                all.into_iter().rev().take(7).rev().collect()
            })
            .unwrap_or_default();

        if rank_history.is_empty() {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("No rank history found for `{}`... :(", username)),
            )
            .await?;
            return Ok(());
        }

        all_users.push(UserData {
            username,
            uid,
            rank_history,
        });
    }

    let max_len = all_users
        .iter()
        .map(|u| u.rank_history.len())
        .max()
        .unwrap_or(1);

    let labels: Vec<String> = (0..max_len)
        .map(|i| {
            let days_ago = max_len - 1 - i;
            match days_ago {
                0 => "now".to_string(),
                1 => "1 day ago".to_string(),
                n => format!("{} days ago", n),
            }
        })
        .collect();

    let datasets: Vec<serde_json::Value> = all_users
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let color = colors[i % colors.len()];
            serde_json::json!({
                "data": u.rank_history,
                "label": u.username,
                "pointStyle": "circle",
                "pointBackgroundColor": color,
                "pointBorderWidth": 0,
                "pointRadius": 4,
                "borderWidth": 4,
                "borderColor": color,
                "fill": {
                    "target": "start",
                    "above": format!("rgba({},{},{},0.15)",
                        i64::from_str_radix(&color[1..3], 16).unwrap_or(0),
                        i64::from_str_radix(&color[3..5], 16).unwrap_or(0),
                        i64::from_str_radix(&color[5..7], 16).unwrap_or(0),
                    )
                },
                "tension": 0.15
            })
        })
        .collect();

    let chart_config = serde_json::json!({
        "type": "line",
        "backgroundColor": "#2a2227",
        "data": {
            "labels": labels,
            "datasets": datasets
        },
        "options": {
            "layout": { "padding": 20 },
            "plugins": {
                "legend": { "labels": { "usePointStyle": true } },
                "datalabels": { "display": false }
            },
            "scales": {
                "x": {
                    "display": true,
                    "grid": { "color": "#382e32" },
                    "ticks": { "color": "#e0b8ca" }
                },
                "y": {
                    "reverse": true,
                    "grace": "10%",
                    "grid": { "display": false },
                    "ticks": { "color": "#e0b8ca" }
                }
            }
        }
    })
    .to_string();

    let chart_url = QuickchartClient::new()
        .width(800)
        .height(500)
        .background_color("#2a2227".to_string())
        .version("4".to_string())
        .chart(chart_config)
        .get_short_url()
        .await?;

    let description = all_users
        .iter()
        .enumerate()
        .map(|(_i, u)| {
            let rank_1 = u.rank_history[0];
            let rank_last = *u.rank_history.last().unwrap();
            let diff = rank_1 - rank_last;
            let change = if diff > 0 {
                format!("<:up:1484520685140447313> {}", format_num(diff as u64))
            } else if diff < 0 {
                format!(
                    "<:down:1484520683924095017> {}",
                    format_num(diff.unsigned_abs())
                )
            } else {
                "<:none:1484585142872965272> No change".to_string()
            };
            let emoji = c_emojis[_i % c_emojis.len()];
            format!(
                "{} **[{}](https://osu.ppy.sh/users/{})** • {} ({} → {})",
                emoji,
                u.username,
                u.uid,
                change,
                format_num(rank_1 as u64),
                format_num(rank_last as u64)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let title = if all_users.len() == 1 {
        format!(
            "<:osu:1482134509729349812> {}'s rank history",
            all_users[0].username
        )
    } else {
        let names: Vec<&str> = all_users.iter().map(|u| u.username.as_str()).collect();
        format!(
            "<:osu:1482134509729349812> {} rank comparison",
            names.join(", ")
        )
    };

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .image(&chart_url)
        .color(0xFF66AA);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
