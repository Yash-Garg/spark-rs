use crate::Context;

#[poise::command(owners_only, prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}
