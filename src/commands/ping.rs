use serenity::{
    builder::CreateApplicationCommand,
    client::Context,

    model::application::interaction::{
                application_command::ApplicationCommandInteraction,
                InteractionResponseType,
    },
    model::prelude::interaction::application_command::CommandDataOption,
};

pub async fn run(_: &[CommandDataOption], ctx: &Context, cmd: &ApplicationCommandInteraction) {
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.content("You called?")
            })
        }).await 
    {
        println!("Error sending message: {}", why);
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
