use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        channel::Message,
        interactions::{
            application_command::ApplicationCommandInteraction,
            message_component::MessageComponentInteraction,
        },
    },
};

pub mod parsable;

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingOption,
    InvalidOption,
    UnknownCommand,
}

#[derive(Debug, Clone)]
pub struct InvocationError;

// To be derivable via macro
pub trait Command: ApplicationCommandInteractionHandler + Sized {
    fn parse(command: &ApplicationCommandInteraction) -> Result<Self, ParseError>;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

#[async_trait]
pub trait ApplicationCommandInteractionHandler {
    async fn invoke(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), InvocationError>;
}

#[async_trait]
pub trait MessageComponentInteractionHandler {
    async fn invoke(
        &mut self,
        ctx: &Context,
        interaction: &MessageComponentInteraction,
        original_message: &mut Message,
    );
}

#[macro_export]
macro_rules! register_commands {
    ($ctx:expr, $guild_id:expr, [$($cmdType:ty),+]) => {{
        if let Some(guild_id) = $guild_id {
            GuildId::set_application_commands(&guild_id, &$ctx.http, |commands_builder| {
                commands_builder
                $(
                    .create_application_command(|command| <$cmdType as slash_helper::Command>::register(command))
                )*
            })
            .await
        } else {
            serenity::model::interactions::application_command::ApplicationCommand::set_global_application_commands(&$ctx.http, |commands_builder| {
                commands_builder
                $(
                    .create_application_command(|command| <$cmdType as slash_helper::Command>::register(command))
                )*
            })
            .await
        }
    }};
}
