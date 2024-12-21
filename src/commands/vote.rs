use serenity::builder::CreateCommand;

pub fn run() -> String {
    "Vote to receive one free spark every 12 hours! Vote here: https://top.gg/bot/1255782111580000349/vote".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("vote").description("One free spark post voting")
}
