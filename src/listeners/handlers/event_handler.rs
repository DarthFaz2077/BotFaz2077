use crate::listeners::events::{message, ready, resume};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
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

    async fn message(&self, ctx: Context, new_message: Message) {
        message::message(&ctx, new_message).await;
    }
}
