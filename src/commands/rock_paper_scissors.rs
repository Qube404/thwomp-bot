#![allow(unused_imports)]
use std::{time::Duration, fmt::format};

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
    let _content = cmd.create_interaction_response(&ctx.http, |response| {
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
                                                       f.create_option(|o| o.label("Scissors").value("Scissors"))
                                                   })
                                               })
                                           })
                                       }))
    }).await.unwrap();

    let message = cmd
        .get_interaction_response(&ctx.http)
        .await.unwrap();

    let interaction = 
        match message.await_component_interaction(&ctx).timeout(Duration::from_secs(60 * 3)).await {
            Some(x) => x,
            None => {
                message.reply(&ctx, "Timed out").await.unwrap();
                return;
            },
        };

    let object2 = &interaction.data.values[0];

    if let Err(why) = cmd.delete_original_interaction_response(&ctx.http).await {
        println!("Cannot delete interaction response: {}", why);
    }

    //******Game Logic Section******//
    let mut winner: &String = object1;
    let mut loser: &String = object2;
    let mut phrase: String = "ties with".to_string();
    if object1 == "Rock" {
        if object2 == "Paper" {
            winner = object2;
            loser = object1;
            phrase = "wraps around".to_string();
        } else if object2 == "Scissors" {
            winner = object1;
            loser = object2;
            phrase = "crushes".to_string();
        }
    } else if object1 == "Paper" {
        if object2 == "Scissors" {
            winner = object2;
            loser = object1;
            phrase = "cuts up".to_string();
        } else if object2 == "Rock" {
            winner = object1;
            loser = object2;
            phrase = "wraps around".to_string();
        }
    } else if object1 == "Scissors" {
        if object2 == "Rock" {
            winner = object2;
            loser = object1;
            phrase = "crushes".to_string();
        } else if object2 == "Paper" {
            winner = object1;
            loser = object2;
            phrase = "cuts up".to_string();
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
    command.name("rock_paper_scissors").description("Get a user id")
        .create_option(|option| {
            option
                .name("rock_paper_scissors")
                .description("Play rock paper scissors")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Rock", "Rock")
                .add_string_choice("Paper", "Paper")
                .add_string_choice("Scissors", "Scissors")
        })

        .create_option(|option| {
            option
                .name("user")
                .description("User to challenge")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
