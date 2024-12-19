use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("Get help with the bot")
}
