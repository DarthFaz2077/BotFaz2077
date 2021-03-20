use crate::models::bot::data::StartTime;
use chrono::Utc;
use humantime::format_duration;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Check bot's uptime."]
#[example("")]
#[owners_only]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let start_time = data.get::<StartTime>().unwrap();
    let uptime = format_duration(start_time.elapsed().unwrap());

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Uptime");
                e.description(uptime);
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
