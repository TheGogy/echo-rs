use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.pause();

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xa6e3a1)
                        .title(":pause_button: Paused!")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xf38ba8)
                        .title(":warning: Not in a voice channel.")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    }
    Ok(())
}

