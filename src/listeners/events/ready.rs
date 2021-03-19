use crate::models::bot::data::{BotConfig, BotVersion};
use serenity::{
    client::Context,
    model::gateway::{Activity, Ready},
};
use tracing::info;

pub async fn ready(ctx: Context, ready: Ready) {
    let data = ctx.data.read().await;
    info!("Connected as {}!", ready.user.name);
    info!("Version: {}", &data.get::<BotVersion>().unwrap());
    let activity = &data.get::<BotConfig>().unwrap().activity;
    ctx.set_activity(Activity::playing(activity)).await;
}
