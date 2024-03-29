#![allow(unused_imports)]
use std::{time::Duration, fmt::format};
use std::collections::HashMap;

use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::{
    builder::{CreateApplicationCommand, CreateButton, CreateComponents, CreateInteractionResponseData, CreateInteractionResponse},
    utils::MessageBuilder,
    client::{Context, EventHandler},
    futures::StreamExt,

    model::{
        prelude::*,
        application::{
            interaction::{
                application_command::ApplicationCommandInteraction,
                InteractionResponseType,
            },
            component::ButtonStyle,
        },
    },

    model::prelude::*,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::CommandDataOption,
    },

    prelude::*,
};

pub async fn run(options: &[CommandDataOption], ctx: &Context, cmd: &ApplicationCommandInteraction) {
    //******Json Value Extraction Section******//
    let mut chars;

    let object_unedited = &options[0].value.as_ref().unwrap().to_string();

    // Removes starting and ending double quotes.
    chars = object_unedited.chars();
    chars.next();chars.next_back();
    let object1 = &chars.as_str().to_string();

    let user_unedited = &options[1].value.as_ref().unwrap().to_string();

    // Removes starting and ending double quotes.
    chars = user_unedited.chars();
    chars.next();chars.next_back();
    let user = &chars.as_str().to_string();

    println!("Object: {}\nUser: {}", object1, user);

    //******Discord API Interaction Section******//
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message
                                       .content(format!("<@{}> you have been challenged to Rock, Paper, Scissors by <@{}>!", user, cmd.user.id))
                                       .components(|c| {
                                           c.create_action_row(|row| {
                                               row.create_select_menu(|menu| {
                                                   menu.custom_id("object_select");
                                                   menu.placeholder("No object selected");
                                                   menu.options(|f| {
                                                        f.create_option(|o| o.label("Rock").value("Rock"));
                                                        f.create_option(|o| o.label("Paper").value("Paper"));
                                                        f.create_option(|o| o.label("Scissors").value("Scissors"));
                                                        f.create_option(|o| o.label("Cowboy").value("Cowboy"));
                                                        f.create_option(|o| o.label("Virus").value("Virus"));
                                                        f.create_option(|o| o.label("Computer").value("Computer"));
                                                        f.create_option(|o| o.label("Thwomp").value("Thwomp"))
                                                   })
                                               })
                                           })
                                       }))
    }).await {
        println!("Cannot create interaction response: {}", why);
    }

    let message = cmd
        .get_interaction_response(&ctx.http)
        .await.unwrap();

    let mut interaction_stream =
        message.await_component_interactions(&ctx).author_id(user.parse::<u64>().unwrap()).timeout(Duration::from_secs(60 * 3)).build();

    let object2 = &interaction_stream.next().await.unwrap().data.values[0];
    message.delete(&ctx).await.unwrap();
    
    //******Game Logic Section******//
    let mut winner: &String = object1;
    let mut loser: &String = object2;
    let mut phrase: String = "ties with".to_string();

    let mut rps_choices: HashMap<&str, [[&str; 2]; 3]> = HashMap::new();
    rps_choices.insert("Rock", [["Virus", "outwaits"], ["Computer", "smashes"], ["Scissors", "crushes"]]);
    rps_choices.insert("Cowboy", [["Scissors", "puts away"], ["Thwomp", "cripples"], ["Rock", "steel-toe kicks"]]);
    rps_choices.insert("Scissors", [["Paper", "cuts"], ["Computer", "cuts cord of"], ["Virus", "cuts DNA of"]]);
    rps_choices.insert("Virus", [["Cowboy", "infects"], ["Computer", "corrupts"], ["Thwomp", "infects"]]);
    rps_choices.insert("Computer", [["Cowboy", "electrocutes"], ["Paper", "uninstalls firmware for"], ["Thwomp", "deletes assets for"]]);
    rps_choices.insert("Thwomp", [["Paper", "pierces"], ["Rock", "shatters"], ["Scissors", "bends"]]);
    rps_choices.insert("Paper", [["Virus", "ignores"], ["Cowboy", "gives papercut too"], ["Rock", "wraps and traps"]]);

    for (k, v) in rps_choices.iter() {
        if object1 == k {
            for item in v {
                if object2 == item[0] {
                    winner = object1;
                    loser = object2;
                    phrase = item[1].to_string();
                }
            }
        } else if object2 == k {
            for item in v {
                if object1 == item[0] {
                    winner = object2;
                    loser = object1;
                    phrase = item[1].to_string();
                }
            }
        }
    }

    let mut winner_user = &cmd.user.id.to_string();
    if winner == object2 {
        winner_user = user;
    }

    let mut win_message = format!("**{}** {} **{}**.\n<@{}> wins!", winner, phrase, loser, winner_user);
    if object1 == object2 {win_message = format!("**{}** {} **{}**.\n         It's a tie!", winner, phrase, loser);}
    if let Err(why) = cmd.channel_id.send_message(&ctx.http, |message| {
        message.content(win_message)
    }).await 
    {
        println!("Cannot send message to channel: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rock_paper_scissors").description("Play rock paper scissors.")

        // Set Options > Create Option because it automatically deletes options you dont' want
        // tacked on to the bot.
        .set_options(vec![
            CreateApplicationCommandOption::default()
                .name("object")
                .description("Object to fight with")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Rock", "Rock")
                .add_string_choice("Paper", "Paper")
                .add_string_choice("Scissors", "Scissors")
                .add_string_choice("Cowboy", "Cowboy")
                .add_string_choice("Virus", "Virus")
                .add_string_choice("Computer", "Computer")
                .add_string_choice("Thwomp", "Thwomp")
                .to_owned(),

            CreateApplicationCommandOption::default()
                .name("user")
                .description("User to challenge")
                .kind(CommandOptionType::User)
                .required(true)
                .to_owned(),
        ])
}
