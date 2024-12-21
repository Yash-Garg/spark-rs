mod commands;

use std::env;

use serenity::all::{CreateEmbed, CreateEmbedFooter};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
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

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
