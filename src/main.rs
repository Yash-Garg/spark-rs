use std::env;

mod commands;
mod constants;

use commands::{ping::ping, spark::spark, vote::vote};
use constants::{BOT_KEY, DB_NAME};
use poise::serenity_prelude as serenity;

pub struct Bot {
    db: sqlx::SqlitePool,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Bot, Error>;

async fn on_error(error: poise::FrameworkError<'_, Bot, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var(BOT_KEY).expect("Expected a token in the environment");

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(DB_NAME)
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    let intents = serenity::GatewayIntents::privileged();
    let options = poise::FrameworkOptions {
        commands: vec![ping(), spark(), vote()],
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                ctx.set_presence(
                    Some(serenity::ActivityData::custom("`/spark` to get started")),
                    serenity::all::OnlineStatus::DoNotDisturb,
                );

                Ok(Bot { db })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
