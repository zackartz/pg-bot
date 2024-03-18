use serenity::all::CreateCommandOption;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Running code...".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("code")
        .description("Run some code")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "language",
            "The language of your program",
        ))
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "code",
            "The source code to run",
        ))
}
