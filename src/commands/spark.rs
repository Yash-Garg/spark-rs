use crate::Context;
use poise::{
    serenity_prelude::{self as serenity, colours::branding::GREEN, CreateEmbedFooter},
    ChoiceParameter,
};

#[derive(Clone, Copy, Debug, PartialEq, poise::ChoiceParameter)]
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

// TODO
impl Compliments {
    fn message(&self) -> &str {
        return match self {
            Compliments::Crush => "",
            Compliments::GreenFlag => "",
            Compliments::RelationshipMaterial => "",
            Compliments::Cutiee => "",
            Compliments::Funny => "",
            Compliments::Smart => "",
            Compliments::Kind => "",
            Compliments::Respect => "",
            Compliments::Miss => "",
            Compliments::Date => "",
            Compliments::PotentialHomie => "",
            Compliments::WRookie => "",
            Compliments::Cancel => "",
            Compliments::Yeet => "",
            Compliments::Mute => "",
            Compliments::Report => "",
        };
    }
}

#[poise::command(
    description_localized(
        "en-US",
        "Do you see a spark in someone? Go ahead, give them a compliment!"
    ),
    // 1 day cooldown
    member_cooldown = 86400,
    ephemeral,
    guild_only,
    prefix_command,
    slash_command
)]
pub async fn spark(
    ctx: Context<'_>,
    #[description = "The user to give a compliment to"] user: serenity::User,
    #[description = "The compliment to give"] compliment: Compliments,
) -> Result<(), anyhow::Error> {
    if user.id == ctx.author().id {
        ctx.say("sussy baka, you can't spark yourself :3").await?;
        return Err(anyhow::anyhow!("User tried to spark themselves"));
    }

    let uid = user.id.get() as i32;
    let db = &ctx.data().db;

    db.add_spark(uid, compliment as i32).await?;

    let sparks_count: i64 = db.get_sparks_count(uid).await?;

    // TODO: Add spark to db

    let message = format!(
        "You've sparked {}!
Your streak is now x{} 🔥
Your daily spark is done, you can spark again tomorrow
Vote to spark again by `/vote`
",
        user.display_name(),
        sparks_count,
    );

    let reply = poise::CreateReply::default().content(message);
    ctx.send(reply).await?;

    // TODO: Figure out guild name
    let embed_msg = format!(
        "<@{}> Someone from {} sparked you {}. Subscribe to [Spark Premium](https://google.com) to reveal!",
        user.id.get(),
        "GUILD_NAME",
        compliment.name()
    );

    let embed_author =
        serenity::CreateEmbedAuthor::new(compliment.name()).icon_url(ctx.author().face());

    let embed = serenity::CreateEmbed::new()
        .author(embed_author)
        .title(format!(
            "🚨 Spark #000{} Someone sparked {} 🚨",
            sparks_count,
            user.display_name()
        ))
        .description(&embed_msg)
        .footer(CreateEmbedFooter::new("Sent by Spark"))
        .color(GREEN)
        .thumbnail(user.face());

    let result = ctx
        .guild_channel()
        .await
        .unwrap()
        .send_message(&ctx.http(), serenity::CreateMessage::new().embed(embed))
        .await;

    if let Err(e) = result {
        return Err(anyhow::anyhow!("Failed to send spark message: {:?}", e));
    }

    Ok(())
}

#[test]
fn test_spark() {
    // TODO
}
