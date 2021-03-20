use serenity::{client::Context, model::event::ResumedEvent};
use tracing::debug;

pub async fn resume(_ctx: Context, resume: ResumedEvent) {
    debug!("Resumed; trace:{:?}", resume.trace)
}
