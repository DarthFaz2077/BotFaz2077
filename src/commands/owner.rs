use crate::{ShardManagerContainer, StartTime};
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
    msg.channel_id
        .say(
            ctx,
            format!(
                "Uptime: {} seconds!",
                start_time.elapsed().unwrap().as_secs()
            ),
        )
        .await?;

    Ok(())
}
