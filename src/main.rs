mod commands;

use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing discord token! please make a .env file in the root of this project and add DISCORD_TOKEN=DISCORD TOKEN HERE to it!");

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping(), commands::shutdown(), commands::rgif()],

            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(">".into()),

                additional_prefixes: vec![
                    poise::Prefix::Literal("yo patchbot"),
                    poise::Prefix::Literal("yo, patchbot"),
                    poise::Prefix::Literal("yo, patchbot,"),
                    poise::Prefix::Literal("yo patchbot,"),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
