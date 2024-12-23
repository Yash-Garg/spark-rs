use serenity::builder::CreateCommand;

pub fn run() -> String {
    "Activating for your GUILD".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("activate").description("Activate the spark bot")
}
