use core::str::FromStr;
use derive_more::{Display, From, FromStr};
use poise::{
    async_trait,
    serenity_prelude::{
        ArgumentConvert, CommandInteraction, Context, CreateCommandOption, ResolvedValue,
    },
    SlashArgError, SlashArgument,
};

#[derive(Clone, Debug, From, FromStr, Display)]
pub struct DefaultSlash<T: Default + FromStr>(pub T);

#[async_trait]
impl<T: Default + FromStr> SlashArgument for DefaultSlash<T>
where
    Option<T>: ArgumentConvert + SlashArgument + Sync,
{
    async fn extract(
        ctx: &Context,
        interaction: &CommandInteraction,
        value: &ResolvedValue<'_>,
    ) -> Result<Self, SlashArgError>
where {
        Ok(DefaultSlash(
            poise::extract_slash_argument!(Option<T>, ctx, interaction, value)
                .await?
                .unwrap_or_default(),
        ))
    }

    fn create(builder: CreateCommandOption) -> CreateCommandOption {
        poise::create_slash_argument!(Option<T>, builder)
    }
}
