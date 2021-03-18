use serde::Deserialize;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use std::{sync::Arc, time::SystemTime};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct StartTime;

impl TypeMapKey for StartTime {
    type Value = SystemTime;
}

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub prefix: String,
    pub activity: String,
}

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = Config;
}

pub struct BotVersion;

impl TypeMapKey for BotVersion {
    type Value = String;
}

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = reqwest::Client;
}
