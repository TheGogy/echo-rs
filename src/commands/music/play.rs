use regex::Regex;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::input::Restartable;
use tokio::process::Command;
use tracing::{error, info};

use crate::commands::utils::to_time;

#[command]
#[aliases(p)]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(0xf38ba8)
                            .title(":warning: Use the command like this: play <url>")
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;
            return Ok(());
        }
    };

    let search = args.clone();

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();


    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        if !url.starts_with("http") {
            let source = match songbird::input::ytdl_search(search.message()).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);

                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.colour(0xf38ba8)
                                    .title(":warning: Error adding song to playlist")
                                    .description("This could mean that one of the songs in the playlist is unavailable.")
                                    .timestamp(Timestamp::now())
                            })
                        })
                        .await?;
                    return Ok(());
                },
            };

            let song = handler.enqueue_source(source.into());
            let mut i = 0;
            for queued_song in handler.queue().current_queue() {
                i += queued_song.metadata().duration.unwrap().as_secs();
            }

            let playtime = to_time(i);
            let metadata = song.metadata();

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(0xa6e3a1)
                            .title(":notes: Found song!")
                            .thumbnail(metadata.thumbnail.clone().unwrap_or_else(|| String::from("https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3")))
                            .description(format!(
                                "{} - {}",
                                metadata.title.clone().unwrap(),
                                metadata.artist.clone().unwrap()
                            ))
                            .fields(vec![
                                ("Songs queued", format!("{}", handler.queue().len()), true),
                                ("Total playtime", playtime, true)
                            ])
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;

        } else if url.contains("playlist") {
            let get_raw_list = Command::new("yt-dlp")
                .args(&["-j", "--flat-playlist", &url])
                .output()
                .await;

            let raw_list = match get_raw_list {
                Ok(list) => String::from_utf8(list.stdout).unwrap(),
                Err(_) => String::from("Error!")
            };

            // Faster than serde somehow
            let re = Regex::new(r#""url": "(https://www.youtube.com/watch\?v=[A-Za-z0-9]{11})""#).unwrap();
            let urls: Vec<String> = re.captures_iter(&raw_list)
                .map(|cap| cap[1].to_string())
                .collect();

            for url in urls {
                info!("Queueing --> {}", url);
                let source = match Restartable::ytdl(url, true).await {
                    Ok(source) => source,
                    Err(why) => {
                        error!("Err starting source: {:?}", why);

                        msg.channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.colour(0xf38ba8)
                                        .title(":warning: Error adding song to playlist")
                                        .description("This could mean that one of the songs in the playlist is unavailable.")
                                        .timestamp(Timestamp::now())
                                })
                            })
                            .await?;
                        return Ok(());
                    }
                };

                let _song = handler.enqueue_source(source.into());
                let mut i = 0;
                for queued_song in handler.queue().current_queue() {
                    i += queued_song.metadata().duration.unwrap().as_secs();
                }

                let playtime = to_time(i);

                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.colour(0xa6e3a1)
                                .title(":notes: Added playlist!")
                                .fields(vec![
                                    ("Songs queued", format!("{}", handler.queue().len()), true),
                                    ("Total playtime", playtime, true)
                                ])
                                .timestamp(Timestamp::now())
                        })
                    })
                    .await?;
            }
        } else {
            let source = match Restartable::ytdl(url, true).await {
                Ok(source) => source,
                Err(why) => {
                    error!("Err starting source: {:?}", why);

                    msg.channel_id
                        .send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.colour(0xf38ba8)
                                    .title(":warning: Error adding song to playlist.")
                                    .description("This could mean that the song is unavailable.")
                                    .timestamp(Timestamp::now())
                            })
                        })
                        .await?;
                    return Ok(());
                }
            };

            let song = handler.enqueue_source(source.into());
            let mut i = 0;
            for queued_song in handler.queue().current_queue() {
                i += queued_song.metadata().duration.unwrap().as_secs();
            }

            let playtime = to_time(i);
            let metadata = song.metadata();

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(0xa6e3a1)
                            .title(":notes: Added to playlist!")
                            .thumbnail(metadata.thumbnail.clone().unwrap_or_else(|| String::from("https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3")))
                            .description(format!(
                                "{} - {}",
                                metadata.title.clone().unwrap(),
                                metadata.artist.clone().unwrap()
                            ))
                            .fields(vec![
                                ("Songs queued", format!("{}", handler.queue().len()), true),
                                ("Total playtime", playtime, true)
                            ])
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;
        }

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