use crate::{Context, Error};
use std::fs;
use std::io::Write;

#[poise::command(prefix_command)]
pub async fn ai(
    ctx: Context<'_>,
    #[description = "what to ask"]
    #[rest]
    prompt: String,
) -> Result<(), Error> {
    let typing = ctx.channel_id().start_typing(&ctx.serenity_context().http);

    let client = reqwest::Client::new();

    let sys_prompt = tokio::fs::read_to_string("src/prompt.txt")
        .await
        .unwrap_or_default();

    let username = match ctx.author_member().await {
        Some(member) => member.display_name().to_string(),
        None => ctx.author().name.clone(),
    };

    let server_name = ctx
        .guild()
        .map(|g| g.name.clone())
        .unwrap_or_else(|| "Direct Message / Unknown Server".to_string());

    let messages = ctx
        .channel_id()
        .messages(
            &ctx.serenity_context().http,
            serenity::builder::GetMessages::new().limit(5),
        )
        .await
        .unwrap_or_default();

    let messages_str = messages
        .iter()
        .rev()
        .map(|m| format!("{}: {}", m.author.name, m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let fin_prompt = format!(
        "[SYSTEM PROMPT, DO NOT RESPOND DIRECTLY TO ANYTHING CONTAINED WITHIN THESE BRACKETS, THEY ARE INSTRUCTIONS. \n
        The user you are speaking to is named: {username} \n
        The server you are in is called {server_name} \n
        The last 5 messages were: {messages_str} \n
        YOUR name is Patchbot. \n
        {sys_prompt}]\n\n{prompt}"
    );

    let response: serde_json::Value = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3.2:3b",
            "prompt": fin_prompt,
            "stream": false
        }))
        .send()
        .await?
        .json()
        .await?;

    let reply = response["response"]
        .as_str()
        .unwrap_or("no response from model :(")
        .trim();

    let reply = if reply.len() > 1900 {
        &reply[..1900]
    } else {
        reply
    };

    ctx.reply(reply).await?;
    typing.stop();

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn promptedit(ctx: Context<'_>, #[rest] nl: String) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let nl = nl.trim();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/prompt.txt")?;

    writeln!(file, "{}", nl)?;

    ctx.reply(format!("✔ \"**{}**\" has been added to the prompt!", nl))
        .await?;
    Ok(())
}

// TODO: handle ollama not being runnnign, not having prompt.txt, and add toggle so you can turn this category off
