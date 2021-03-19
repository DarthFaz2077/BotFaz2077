use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Avatar");
                    e.url(msg.author.face());
                    e.description(msg.author.tag());
                    e.image(msg.author.face());
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face());

                        f
                    });
                    e.timestamp(&Utc::now());

                    e
                });

                m
            })
            .await?;
    } else {
        for user in &msg.mentions {
            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Avatar");
                        e.url(user.face());
                        e.description(user.tag());
                        e.image(user.face());
                        e.footer(|f| {
                            f.text(format!("Requested by {}", msg.author.tag()));
                            f.icon_url(msg.author.face());

                            f
                        });
                        e.timestamp(&Utc::now());

                        e
                    });

                    m
                })
                .await?;
        }
    }

    Ok(())
}
