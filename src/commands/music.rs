use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use songbird::input::Restartable;

#[command]
#[aliases(j)]
#[max_args(1)]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e
                    .colour(0xf38ba8)
                    .title(":warning: Join a voice channel first!")
                    .timestamp(Timestamp::now())
                )
            }).await?;

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (_, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xa6e3a1)
                .title(format!("Joined channel --> {}", connect_to.mention()))
                .timestamp(Timestamp::now())
            )
        }).await?;
    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: error joining channel.")
                .description("Please ensure I have the correct permissions.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            println!("Failed to join voice channel: {}", e);
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e
                    .colour(0xf38ba8)
                    .title("Failed to join voice channel.")
                    .timestamp(Timestamp::now())
                )
            }).await?;
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xa6e3a1)
                .title("Left voice channel!")
                .timestamp(Timestamp::now())
            )
        }).await?;

    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Not in a voice channel.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Use the command like this: `play <url>`")
                .await?;
            return Ok(());
        },
    };

    if !url.starts_with("http") {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Please enter a valid URL.")
                .timestamp(Timestamp::now())
            )
        }).await?;

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Use Restartable::ytdl here, otherwise queue seems to stop working
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| e
                        .colour(0xf38ba8)
                        .title(":warning: Error sourcing yt-dlp.")
                        .timestamp(Timestamp::now())
                    )
                }).await?;

                return Ok(());
            },
        };

        let _song = handler.enqueue_source(source.into());
        msg.channel_id.say(&ctx.http, format!("Playing song. Position in queue: {}", handler.queue().len())).await?;

    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Not in a voice channel.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xa6e3a1)
                .title(":track_next: Skipped!")
                .timestamp(Timestamp::now())
            )
        }).await?;

    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Not in a voice channel.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xa6e3a1)
                .title(":pause_button: Paused!")
                .timestamp(Timestamp::now())
            )
        }).await?;

    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Not in a voice channel.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.resume();

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xa6e3a1)
                .title(":arrow_forward: Resumed!")
                .timestamp(Timestamp::now())
            )
        }).await?;

    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .colour(0xf38ba8)
                .title(":warning: Not in a voice channel.")
                .timestamp(Timestamp::now())
            )
        }).await?;
    }

    Ok(())
}