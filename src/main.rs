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

const HELP_MESSAGE: &str = "
Hello there, Human!
Nice to meet you, I am thwomp bot.
❓What can I do?
➡️ Currently my functionality is limited due to my lazy developer
➡️ but the three things I should be able to do is play 7-item rock
➡️ paper scissors, read and calculate text equations and create
➡️ reaction votes for your messages.

❓Something not working?
➡️ Sounds like a you problem.

I hope you have a great day, and don't forget to milk the milkman!
— ThwompBot 🤖";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "kys" || msg.content == "KYS" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "No you kill yourself you lonely fucking loser.").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Recieved command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "test" => commands::test::run(&command.data.options, &ctx, &command).await,
                "ping" => commands::ping::run(&command.data.options, &ctx, &command).await,
                "rock_paper_scissors" => commands::rock_paper_scissors::run(&command.data.options, &ctx, &command).await,
                "qalc" => commands::qalculator::run(&command.data.options, &ctx, &command).await,

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
            commands::test::register(command);
            commands::ping::register(command);
            commands::rock_paper_scissors::register(command);
            commands::qalculator::register(command)
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
