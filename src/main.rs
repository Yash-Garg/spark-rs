#![allow(unused)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod commands;
mod constants;
mod db;

use anyhow::Ok;
use commands::{activate::activate, ping::ping, spark::spark, vote::vote};
use constants::{BOT_KEY, DB_KEY};
use db::manager::DbManager;
use poise::serenity_prelude as serenity;
use std::env;

pub struct Bot {
    db: DbManager,
}

type Context<'a> = poise::Context<'a, Bot, anyhow::Error>;

async fn on_error(error: poise::FrameworkError<'_, Bot, anyhow::Error>) {
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
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap_or_default();
    dotenv::dotenv().expect("Couldn't load .env file");

    let token = env::var(BOT_KEY).expect("Expected a token in the environment");
    let db_url = env::var(DB_KEY).expect("Expected a database URL in the environment");

    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run database migrations");

    let intents = serenity::GatewayIntents::privileged();
    let options = poise::FrameworkOptions {
        commands: vec![activate(), ping(), spark(), vote()],
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

                Ok(Bot {
                    db: DbManager::new(pool),
                })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.unwrap().start().await {
        println!("Err with client: {:?}", why);
    }

    Ok(())
}
