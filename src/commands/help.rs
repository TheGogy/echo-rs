use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::model::Timestamp;
use serenity::prelude::*;

use std::env;

// Custom help menu, might use inbuilt one later. For noe, this seems like the better choice

#[command]
pub async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix = env::var("PREFIX").expect("Set your PREFIX environment variable!");

    let menu_choice_str: String = match args.single::<String>() {
        Ok(menu_choice) => menu_choice,
        Err(_) => "default".to_string(),
    };

    let menu_choice: &str = &menu_choice_str;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e
            .colour(0x89dceb)
            .thumbnail("https://images.unsplash.com/photo-1574169208507-84376144848b?ixlib=rb-4.0.3")
            .title("**- -【 Ｈｅｌｐ 】- -**")
            .description(format!("Hi! I'm Echo. A simple rust discord bot made with Serenity and Songbird! My prefix is `{}`.", prefix))
            .fields(
                match menu_choice {

                    "general" => {
                        vec![
                            ("help", "Displays this help menu", true),
                            ("ping", "Shows info about the bot", true),
                            ("choice", "Selects a random number from a given range", true),
                        ]
                    },

                    "music" => {
                        vec![
                            ("join", "Joins a voice channel", true),
                            ("leave", "Leaves a music channel", true),
                            ("play", "Play / queue a song from a YouTube URL", true),
                            ("stop", "Stops current playlist", true),
                            ("skip", "Skips the current song", true),
                            ("pause", "Pauses the current song", true),
                            ("resume", "Resumes the current song", true),
                            ("nowplaying", "Shows info about current song", true),
                            ("queue", "Show the current queue", true),
                            ("shuffle", "Shuffles the current playlist", true),
                        ]
                    },

                    _ => {
                        vec![
                            ("help", "Displays this help menu", false),
                            ("help music", "Show music commands", false),
                            ("help general", "Show general commands", false),
                        ]
                    },
                }
            )
            .footer(|f| f.text("Made by Gogy"))
            .timestamp(Timestamp::now())
        )
    }).await?;

    Ok(())
}
