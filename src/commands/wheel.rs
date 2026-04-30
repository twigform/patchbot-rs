use crate::{Context, Error};
use rand::seq::IteratorRandom;
use std::fs;
use std::io::Write;

#[poise::command(prefix_command)]
pub async fn wheel(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    if fs::exists("src/wheel.txt").unwrap_or(false) {
        let contents = fs::read_to_string("src/wheel.txt")?;
        if contents.trim().is_empty() {
            let response = "wheel.txt exists, but it's empty... (߹𖥦߹) \nrun >wheeladd [input] to add a new entry!";
            ctx.reply(response).await?;
        } else {
            let line = contents.lines().choose(&mut rand::rng()).unwrap();
            ctx.reply(line).await?;
            return Ok(());
        }
    } else {
        fs::File::create("src/wheel.txt")?;
        let response = "wheel doesn't exist, so i made it! (๑・ω・๑) \nrun >wheeladd [input] to add a new entry!";
        ctx.reply(response).await?;
        return Ok(());
    }
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn wheeladd(ctx: Context<'_>, #[rest] nl: String) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(&ctx.http()).await?;

    let nl = nl.trim();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/wheel.txt")?;

    writeln!(file, "{}", nl)?;

    ctx.reply(format!("✔ \"**{}**\" has been added to the wheel!", nl))
        .await?;
    Ok(())
}
