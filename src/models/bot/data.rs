use crate::models::bot::config::Config;
use reqwest::Client as ReqwestClient;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use sqlx::PgPool;
use std::{sync::Arc, time::SystemTime};

pub struct ShardManagerContainer;
pub struct StartTime;
pub struct BotConfig;
pub struct ReqwestClientContainer;
pub struct PgPoolContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for StartTime {
    type Value = SystemTime;
}

impl TypeMapKey for BotConfig {
    type Value = Config;
}

impl TypeMapKey for ReqwestClientContainer {
    type Value = ReqwestClient;
}

impl TypeMapKey for PgPoolContainer {
    type Value = PgPool;
}
