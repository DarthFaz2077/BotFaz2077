mod commands;

use std::{collections::HashSet, env, sync::Arc};

use log::{error, info};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use commands::{general::*, owners::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed!");
    }
}

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(shutdown)]
struct Owners;

#[tokio::main]
async fn main() {
    kankyo::load().expect("Failed to load .env file!");

    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment!");
    let prefix = env::var("PREFIX").expect("Expected a prefix in the environment!");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {}!", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix))
        .group(&GENERAL_GROUP)
        .group(&OWNERS_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}!", why)
    }
}
