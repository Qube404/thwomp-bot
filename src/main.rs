mod commands;

use std::env;
use dotenv;

use serenity::{
    async_trait,

    model::{
        application::{
        command::Command, 
        interaction::{Interaction, InteractionResponseType},
        }, 
    prelude::*,
    channel::Message, 
    gateway::Ready,
    id::GuildId,
    },
    prelude::*,
    client::{Context, EventHandler},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "kys" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Fuck you.").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Recieved command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options, &ctx, &command).await,
                "rps" => commands::rps::run(&command.data.options, &ctx, &command).await,
                "calc" => commands::calc::run(&command.data.options, &ctx, &command).await,

                _ => if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("Not Implemented")) 
                    }).await
                {
                    println!("Cannot respond to slash command: {}", why);
                },
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expect GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer")
            );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
        }).await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let global_commands = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command);
            commands::rps::register(command);
            commands::calc::register(command)
        }).await;

        println!("I created the following global slash command: {:#?}", global_commands);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
