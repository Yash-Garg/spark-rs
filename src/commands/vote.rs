use crate::Context;

#[poise::command(
    description_localized("en-US", "One free spark post voting"),
    prefix_command,
    slash_command
)]
pub async fn vote(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    ctx.reply("Vote to receive one free spark every 12 hours! Vote here: https://top.gg/bot/1255782111580000349/vote").await?;
    Ok(())
}
