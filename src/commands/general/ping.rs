use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Ping."]
#[example("")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Ping");
                e.description("Pong!");
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
