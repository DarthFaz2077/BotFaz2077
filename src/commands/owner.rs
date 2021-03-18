use crate::{BotVersion, ShardManagerContainer, StartTime};
use chrono::Utc;
use humantime::format_duration;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[owners_only]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Shutdown");
                    e.description("Shutting down");
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
        manager.lock().await.shutdown_all().await;
    } else {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Shutdown");
                    e.description("There was a problem getting the shard manager");
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

    Ok(())
}

#[command]
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

    Ok(())
}

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

    Ok(())
}
