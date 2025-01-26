use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Vote to receive one free spark every 12 hours! Vote here: https://top.gg/bot/1255782111580000349/vote").await?;
    Ok(())
}
