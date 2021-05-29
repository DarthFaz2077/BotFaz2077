use crate::models::bot::data::BotConfig;
use serenity::{
    client::Context,
    model::gateway::{Activity, Ready},
};
use tracing::info;

pub async fn ready(ctx: Context, ready: Ready) {
    let data = ctx.data.read().await;
    info!("Connected as {}!", ready.user.name);
    info!("Version: {}", env!("VERGEN_GIT_SHA_SHORT"));
    let activity = &data.get::<BotConfig>().unwrap().activity;
    ctx.set_activity(Activity::playing(activity)).await;
}
