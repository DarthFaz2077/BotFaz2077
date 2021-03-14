use crate::{BotVersion, ShardManagerContainer, StartTime};
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
        msg.channel_id.say(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.channel_id
            .say(ctx, "There was a problem getting the shard manager!")
            .await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[owners_only]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let start_time = data.get::<StartTime>().unwrap();
    let parsed_time = format_duration(start_time.elapsed().unwrap());

    msg.channel_id
        .say(ctx, format!("Uptime: {}", parsed_time))
        .await?;

    Ok(())
}

#[command]
#[owners_only]
async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let version_hash = data.get::<BotVersion>().unwrap();

    msg.channel_id
        .say(ctx, format!("Version: {}", version_hash))
        .await?;

    Ok(())
}
