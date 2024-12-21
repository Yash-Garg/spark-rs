use serenity::builder::CreateCommand;

pub fn run() -> String {
    "How to use the spark bot?".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("How to use the spark bot?")
}
