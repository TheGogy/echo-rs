mod commands;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{debug, error, info, instrument};

use songbird::SerenityInit;

/* Import commands */
use crate::commands::help::*;
use crate::commands::music::*;
use crate::commands::ping::*;

/* Shards container */
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as --> {} [id: {}]", ready.user.name, ready.user.id);
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Received command --> '{}' || User --> '{}'", command_name, msg.author.name);
    true
}


#[group]
#[commands(
    // Misc
    help,   ping,

    // Music commands
    join,   leave,  play,   pause,  resume,
    skip,

)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file.");

    let token = env::var("DISCORD_TOKEN").expect("Set your DISCORD_TOKEN environment variable!");
    let prefix = env::var("PREFIX").expect("Set your PREFIX environment variable!");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Initialise error tracing
    tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners)
        .prefix(prefix))
        .before(before)
        .group(&GENERAL_GROUP);

        let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}