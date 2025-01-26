use crate::{Context, Error};
use poise::{
    serenity_prelude::{self as serenity, CreateEmbedFooter},
    ChoiceParameter,
};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Compliments {
    #[name = "Crush 💖"]
    Crush,
    #[name = "Green Flag 💚"]
    GreenFlag,
    #[name = "Relationship Material 👩❤👨"]
    RelationshipMaterial,
    #[name = "A cutiee 🤩"]
    Cutiee,
    #[name = "Funny 😁"]
    Funny,
    #[name = "Smart 😎"]
    Smart,
    #[name = "Kind 😇"]
    Kind,
    #[name = "Respect 🫡"]
    Respect,
    #[name = "Miss 🥺"]
    Miss,
    #[name = "Date? ❤"]
    Date,
    #[name = "Potential Homie 🤝🏻"]
    PotentialHomie,
    #[name = "W Rookie 🤟🏻"]
    WRookie,
    #[name = "Cancel 🥱"]
    Cancel,
    #[name = "Yeet 👋🏻"]
    Yeet,
    #[name = "Mute 🤫"]
    Mute,
    #[name = "Report 🚫"]
    Report,
}

#[poise::command(
    description_localized(
        "en-US",
        "Do you see a spark in someone? Go ahead, give them a compliment!"
    ),
    // 1 day cooldown
    // member_cooldown = 86400,
    ephemeral,
    guild_only,
    prefix_command,
    slash_command
)]
pub async fn spark(
    ctx: Context<'_>,
    #[description = "The user to give a compliment to"] user: serenity::User,
    #[description = "The compliment to give"] compliment: Compliments,
) -> Result<(), Error> {
    if user.id == ctx.author().id {
        ctx.say("sussy baka, you can't spark yourself :3").await?;
        return Err("User tried to spark themselves".into());
    }

    let db = &ctx.data().db;
    // TODO: Add spark to db

    let message = format!(
        "You've sparked {}!
Your streak is now x{} 🔥
Your daily spark is done, you can spark again tomorrow
Vote to spark again by `/vote`
",
        user.display_name(),
        1,
    );

    let reply = poise::CreateReply::default().content(message);
    ctx.send(reply).await?;

    // TODO: Figure out guild name
    let embed_msg = format!(
        "Someone from {} sparked you {}. Subscribe to [Spark Premium](https://google.com) to reveal!",
        "GUILD_NAME",
        compliment.name()
    );

    let embed_author =
        serenity::CreateEmbedAuthor::new(compliment.name()).icon_url(ctx.author().face());

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .title(format!(
            "🚨 Spark #{} Someone sparked {} 🚨",
            0001,
            user.display_name()
        ))
        .description(&embed_msg)
        .footer(CreateEmbedFooter::new("Sent by Spark"))
        .color(0x00FF00)
        .thumbnail(user.face());

    let result = ctx
        .send(poise::CreateReply::default().ephemeral(false).embed(embed))
        .await;

    if let Err(e) = result {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

#[test]
fn test_spark() {
    // TODO
}
