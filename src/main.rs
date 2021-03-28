mod commands;
mod listeners;
mod models;
mod utilities;

use crate::listeners::{handlers::event_handler::Handler, hooks::before::before};
use crate::models::bot::{command_groups::*, config::*, data::*};
use crate::utilities::help::*;
use reqwest::Client as ReqwestClient;
use serenity::{framework::standard::StandardFramework, http::Http, prelude::*};
use sqlx::{migrate, PgPool};
use std::{collections::HashSet, fs, time::SystemTime};
use tracing::{error, instrument};

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let start_time = SystemTime::now();

    let config = envy::from_env::<Config>().unwrap();

    let hash = blake3::hash(&fs::read(std::env::current_exe().unwrap()).unwrap());
    let version_hash = hash.to_hex().to_string();

    let reqwest_client = ReqwestClient::new();

    let pg_pool = PgPool::connect(&config.postgres_url).await.unwrap();
    migrate!("./migrations").run(&pg_pool).await.unwrap();

    let http = Http::new_with_token(&config.discord_token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {}!", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&config.prefix))
        .group(&GENERAL_GROUP)
        .group(&ANIME_GROUP)
        .group(&ECONOMY_GROUP)
        .group(&FUN_GROUP)
        .group(&OWNERS_GROUP)
        .help(&HELP)
        .before(before);

    let mut client = Client::builder(&config.discord_token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    {
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<StartTime>(start_time);
        data.insert::<BotConfig>(config);
        data.insert::<BotVersion>(version_hash);
        data.insert::<ReqwestClientContainer>(reqwest_client);
        data.insert::<PgPoolContainer>(pg_pool);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}!", why)
    }
}
