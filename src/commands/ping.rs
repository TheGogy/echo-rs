use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e
                    .colour(0xf38ba8)
                    .title(":warning: No shard found!")
                    .timestamp(Timestamp::now())
                )
            }).await?;

            return Ok(());
        },
    };

    // Latency takes a minute or so to calibrate
    let latency = match runner.latency {
        Some(latency) => format!("{:.2} ms", latency.as_millis()),
        None => "...".to_string(),
    };

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e
            .colour(0x89dceb)
            .title("**Info**")
            .fields(
                vec![
                    ("Shard ID", format!("{:?}", ctx.shard_id), true),
                    ("Latency", latency, true),
                ]
            )
            .field("Status:", runner.stage, false)
            .timestamp(Timestamp::now())
        )
    }).await?;

    Ok(())
}