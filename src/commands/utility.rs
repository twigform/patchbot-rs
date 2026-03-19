use crate::{Context, Error};
use poise::builtins::HelpConfiguration;

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
