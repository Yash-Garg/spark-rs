#![allow(dead_code)]

mod commands;
mod constants;

use std::env;

use constants::{BOT_KEY, DB_NAME, GUILD_KEY};
use serenity::all::{CreateEmbed, CreateEmbedFooter, Message};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Bot {
    token: String,
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "help" => Some(commands::help::run()),
                "spark" => Some(commands::spark::run(&command.data.options())),
                "vote" => Some(commands::vote::run()),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let footer = CreateEmbedFooter::new("Sent by Spark");
                let embed = CreateEmbed::new()
                    .title("Spark")
                    .description(&content)
                    .field("User", command.member.clone().unwrap().display_name(), true)
                    .footer(footer);

                let data = CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(command.data.name == "spark");

                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received Message: {}", msg.content);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var(GUILD_KEY)
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::spark::register(), commands::vote::register()],
            )
            .await;

        if let Err(why) = commands {
            println!("Cannot register commands: {why}");
        }

        let guild_command =
            Command::create_global_command(&ctx.http, commands::help::register()).await;

        if let Err(why) = guild_command {
            println!("Cannot register guild command: {why}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var(BOT_KEY).expect("Expected a token in the environment");

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(DB_NAME)
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // sqlx::migrate!("./migrations")
    //     .run(&database)
    //     .await
    //     .expect("Couldn't run database migrations");

    let bot = Bot { token, database };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&bot.token, intents)
        .event_handler(bot)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
