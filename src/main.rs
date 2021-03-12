mod commands;

use dotenv::dotenv;

use std::{collections::HashSet, env, sync::Arc};

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

use commands::{fun::*, general::*, owner::*};

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
#[commands(shutdown)]
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
    dotenv().ok();

    tracing_subscriber::fmt::init();

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
        .group(&FUN_GROUP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP)
        .help(&MY_HELP)
        .before(before);

    let mut client = Client::builder(&token)
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
