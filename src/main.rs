#![allow(dead_code)]

mod commands;
mod constants;

use std::env;

use constants::{BOT_KEY, DB_NAME};
use serenity::all::{
    ActivityData, CreateEmbed, CreateMessage, Message, MessageInteractionMetadata,
};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Bot {
    token: String,
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "activate" => Some(commands::activate::run()),
                "help" => Some(commands::help::run()),
                "spark" => Some(commands::spark::run(&command.data.options())),
                "vote" => Some(commands::vote::run()),
                _ => Some("sussy baka, it's not implemented :(".to_string()),
            };

            if let Some(content) = content {
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
        println!("Received message: {:#?}", msg);

        match msg.interaction_metadata {
            Some(interaction) => {
                match *interaction {
                    MessageInteractionMetadata::Command(meta) => {
                        let sv_name = msg.guild_id.unwrap().name(&ctx.cache).unwrap();

                        let desc = format!(
                            "Someone from {} sparked you {}. Subscribe to [Spark Premium](https://google.com) to reveal!",
                            sv_name, "compliment"
                        );

                        let embed = CreateEmbed::new()
                            .title("Spark #0001")
                            .description(desc)
                            .color(0x00FF00);

                        let message = CreateMessage::new().embed(embed);
                        meta.user.dm(&ctx.http, message).await.unwrap();

                        // msg.channel_id
                        //     .send_message(&ctx.http, message)
                        //     .await
                        //     .unwrap();
                    }
                    _ => todo!(),
                }
            }
            None => { /* no-op */ }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(
            Some(ActivityData::custom("`/spark` to get started")),
            serenity::all::OnlineStatus::Idle,
        );

        let glob_cmds = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::activate::register(),
                commands::help::register(),
                commands::spark::register(),
                commands::vote::register(),
            ],
        )
        .await;

        if let Err(why) = glob_cmds {
            println!("Cannot register global commands: {why}");
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
