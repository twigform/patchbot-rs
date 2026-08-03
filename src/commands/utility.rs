use crate::{Context, Error};
use poise::builtins::HelpConfiguration;
use reqwest::header::USER_AGENT;

#[poise::command(prefix_command, owners_only, category = "Utility")]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("ok bye :(").await?;
    std::process::exit(0);
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

#[poise::command(prefix_command, category = "Utility")]
pub async fn define(
    ctx: Context<'_>,
    #[description = "word to define"]
    #[rest]
    word: String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response: serde_json::Value = client
        .get(&url)
        .header(USER_AGENT, "patchbot_discord")
        .send()
        .await?
        .json()
        .await?;

    let phonetic = response[0]["phonetic"]
        .as_str()
        .unwrap_or("no pronunciation found");

    let def = response[0]["meanings"][0]["definitions"][0]["definition"]
        .as_str()
        .unwrap_or("no definition found");

    let type_of_word = response[0]["meanings"][0]["partOfSpeech"]
        .as_str()
        .unwrap_or("no word type found");

    let finalmsg = format!("# {} - {} ({}) \n{}", word, phonetic, type_of_word, def);

    ctx.reply(finalmsg).await?;
    Ok(())
}
// maybe add better word not found handling later
