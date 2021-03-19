use crate::models::bot::data::BotVersion;
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[owners_only]
async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let version_hash = data.get::<BotVersion>().unwrap();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Version");
                e.description(version_hash);
                e.footer(|f| {
                    f.text(format!("Requested by {}.", msg.author.tag()));
                    f.icon_url(msg.author.face());

                    f
                });
                e.timestamp(&Utc::now());

                e
            });

            m
        })
        .await?;

    Ok(())
}
