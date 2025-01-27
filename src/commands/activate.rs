use crate::Context;

use poise::serenity_prelude as serenity;

#[poise::command(
    description_localized("en-US", "Set the current channel as the active channel for the bot."),
    ephemeral,
    guild_only,
    prefix_command,
    slash_command
)]
pub async fn activate(
    ctx: Context<'_>,
    #[description_localized("en-US", "The channel to activate.")] channel: Option<
        serenity::GuildChannel,
    >,
) -> Result<(), anyhow::Error> {
    if channel.is_none() {
        ctx.reply("No channel provided").await?;
        return Err(anyhow::anyhow!("No channel provided"));
    }

    let db = &ctx.data().db;
    let reply = serenity::CreateMessage::new()
        .content("All the sparks will be shown in this channel now. Spark anywhere, read here.");

    let did_send = &channel.as_ref().unwrap().send_message(&ctx, reply).await;
    match did_send {
        Ok(_) => {
            ctx.say(format!(
                "Activated {} for all spark messages.",
                channel.as_ref().unwrap().name
            ))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send confirmation message: {:?}", e))?;

            db.set_active_channel(
                ctx.guild_id().unwrap().get() as i64,
                channel.unwrap().id.get() as i64,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to set active channel: {:?}", e))?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to send message: {:?}", e));
        }
    }

    Ok(())
}
