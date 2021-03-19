use serenity::{client::Context, framework::standard::macros::hook, model::channel::Message};
use tracing::info;

#[hook]
#[instrument]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'.",
        command_name, msg.author.name
    );

    true
}
