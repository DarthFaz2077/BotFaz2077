mod commands;

use std::{collections::HashSet, sync::Arc, time::SystemTime};

use tracing::{debug, error, info, instrument};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands, macros::group, macros::help, macros::hook, Args, CommandGroup,
        CommandResult, HelpOptions, StandardFramework,
    },
    http::Http,
    model::prelude::*,
    prelude::*,
};

use serde::Deserialize;

use commands::{fun::*, general::*, owner::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct StartTime;

impl TypeMapKey for StartTime {
    type Value = SystemTime;
}

#[derive(Deserialize)]
struct Config {
    discord_token: String,
    prefix: String,
    activity: String,
    rust_log: String,
}

struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = Config;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}!", ready.user.name);
        let data = ctx.data.read().await;
        let activity = &data.get::<BotConfig>().unwrap().activity;
        ctx.set_activity(Activity::playing(activity)).await;
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace:{:?}", resume.trace)
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'.",
        command_name, msg.author.name
    );

    true
}

#[group]
#[commands(poke, feed, pat)]
struct Fun;

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(shutdown, uptime)]
struct Owner;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = envy::from_env::<Config>().unwrap();

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
        .group(&FUN_GROUP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP)
        .help(&MY_HELP)
        .before(before);

    let mut client = Client::builder(&config.discord_token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    {
        let mut data = client.data.write().await;
        data.insert::<StartTime>(SystemTime::now());
    }

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    {
        let mut data = client.data.write().await;
        data.insert::<BotConfig>(config);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}!", why)
    }
}
