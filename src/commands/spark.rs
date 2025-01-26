use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use strum::EnumIter;

#[derive(Debug, poise::ChoiceParameter, EnumIter)]
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
    let message = format!(
        "You've sparked {}!
Your streak is now x{} 🔥
Your daily spark is done, you can spark again tomorrow
Vote to spark again by `/vote`
",
        user.display_name(),
        1,
    );

    ctx.reply(message).await?;

    let desc = format!(
        "Someone from {} sparked you {:?}. Subscribe to [Spark Premium](https://google.com) to reveal!",
        ctx.guild_id().unwrap().name(&ctx.cache()).unwrap(),
        compliment
    );

    ctx.say(desc.to_string()).await?;

    Ok(())
}
