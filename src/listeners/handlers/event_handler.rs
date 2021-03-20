use crate::listeners::events::{ready, resume};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{event::ResumedEvent, gateway::Ready},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }

    async fn resume(&self, ctx: Context, resume: ResumedEvent) {
        resume::resume(ctx, resume).await;
    }
}
