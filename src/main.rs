mod commands;

use std::env;

use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Ready},
    async_trait,
    model::application::Interaction,
    prelude::*,
};

struct Handler;

/**
 * General idea for flow
 *
 * User Joins -> Bot DMs -> user solves with either /solve <code> or just !solve ```<optional
 * language> <code>``` -> solution is either accepted or denied -> role assigned (or not)
 */

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(858572001907572768); // TODO: hardcoded guild id -- fix this?

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::ping::register(), commands::code::register()],
            )
            .await;

        if commands.is_err() {
            println!("fuck");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "code" => Some(commands::code::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("dotenvy to find token");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {}", why);
    }
}
