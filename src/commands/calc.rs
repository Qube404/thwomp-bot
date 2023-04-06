use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseData},
    client::Context,

    model::application::interaction::{
                application_command::ApplicationCommandInteraction,
                InteractionResponseType,
            },
    model::prelude::interaction::application_command::CommandDataOption,
    model::prelude::command::CommandOptionType,
};

use cli_calculator::Equation;

pub async fn run(options: &[CommandDataOption], ctx: &Context, cmd: &ApplicationCommandInteraction) {
    //******Json Value Extraction Section******//
    let mut chars;

    let calc_unedited = &options[0].value.as_ref().unwrap().to_string();

    // Removes starting and ending double quotes.
    chars = calc_unedited.chars();
    chars.next();chars.next_back();
    let calculation = chars.as_str().to_string();

    //******Calculation Section******//
    let calculation = match Equation::from(calculation) {
        Ok(val) => val,
        Err(why) => {       
            interaction_response(cmd, ctx, true, &format!("{why}")).await;
            return
        }
    };

    interaction_response(cmd, ctx, false, &format!("{} = {}", calculation.get_equation(), calculation.get_result())).await;
}

//******Discord API Interaction Section******//
async fn interaction_response(cmd: &ApplicationCommandInteraction, ctx: &Context, ephemeral: bool, content: &str) {
    if let Err(why) = cmd.create_interaction_response(ctx, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .content(content)
                    .ephemeral(ephemeral)
            })
        }).await
    {
        println!("Cannot respond to slash command: {}", why);
    }
} 

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("calc").description("An automatic text calculator.")
        .set_options(vec![
            CreateApplicationCommandOption::default()
                .name("calculator")
                .description("The equation for the bot to calculate.")
                .kind(CommandOptionType::String)
                .required(true).to_owned()
        ])
}














