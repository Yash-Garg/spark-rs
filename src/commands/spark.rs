use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use strum::EnumIter;

pub fn run(options: &[ResolvedOption]) -> String {
    match (options.get(0), options.get(1)) {
        (
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }),
            Some(ResolvedOption {
                value: ResolvedValue::String(compliment),
                ..
            }),
        ) => format!(
            "Someone sees a spark in you, <@{}>! They think you're {}",
            user.id, compliment
        ),
        _ => "Please provide a valid user and compliment".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("spark")
        .description("Do you see a spark in someone? Go ahead, give them a compliment!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to give a compliment to",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "compliment",
                "The compliment to give",
            )
            .add_string_choice(Compliments::Crush.value(), Compliments::Crush.value())
            .add_string_choice(
                Compliments::GreenFlag.value(),
                Compliments::GreenFlag.value(),
            )
            .add_string_choice(
                Compliments::RelationshipMaterial.value(),
                Compliments::RelationshipMaterial.value(),
            )
            .add_string_choice(Compliments::Cutiee.value(), Compliments::Cutiee.value())
            .add_string_choice(Compliments::Funny.value(), Compliments::Funny.value())
            .add_string_choice(Compliments::Smart.value(), Compliments::Smart.value())
            .add_string_choice(Compliments::Kind.value(), Compliments::Kind.value())
            .add_string_choice(Compliments::Respect.value(), Compliments::Respect.value())
            .add_string_choice(Compliments::Miss.value(), Compliments::Miss.value())
            .add_string_choice(Compliments::Date.value(), Compliments::Date.value())
            .add_string_choice(
                Compliments::PotentialHomie.value(),
                Compliments::PotentialHomie.value(),
            )
            .add_string_choice(Compliments::WRookie.value(), Compliments::WRookie.value())
            .add_string_choice(Compliments::Cancel.value(), Compliments::Cancel.value())
            .add_string_choice(Compliments::Yeet.value(), Compliments::Yeet.value())
            .add_string_choice(Compliments::Mute.value(), Compliments::Mute.value())
            .required(true),
        )
}

#[derive(Debug, EnumIter)]
enum Compliments {
    Crush,
    GreenFlag,
    RelationshipMaterial,
    Cutiee,
    Funny,
    Smart,
    Kind,
    Respect,
    Miss,
    Date,
    PotentialHomie,
    WRookie,
    Cancel,
    Yeet,
    Mute,
}

impl Compliments {
    fn value(&self) -> &str {
        match self {
            Compliments::Crush => "Crush 💖",
            Compliments::GreenFlag => "Green Flag 💚",
            Compliments::RelationshipMaterial => "Relationship Material 👩❤👨",
            Compliments::Cutiee => "A cutiee 🤩",
            Compliments::Funny => "Funny 😁",
            Compliments::Smart => "Smart 😎",
            Compliments::Kind => "Kind 😇",
            Compliments::Respect => "Respect 🫡",
            Compliments::Miss => "Miss 🥺",
            Compliments::Date => "Date? ❤",
            Compliments::PotentialHomie => "Potential Homie 🤝🏻",
            Compliments::WRookie => "W Rookie 🤟🏻",
            Compliments::Cancel => "Cancel 🥱",
            Compliments::Yeet => "Yeet 👋🏻",
            Compliments::Mute => "Mute 🤫",
        }
    }
}
