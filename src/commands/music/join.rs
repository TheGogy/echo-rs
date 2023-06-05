use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases(j)]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{:?}\n{:?}", msg, _args);
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(0xf38ba8)
                            .title(":warning: Join a voice channel first!")
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (_, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xa6e3a1)
                        .title(format!("Joined channel --> {}", connect_to.mention()))
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xf38ba8)
                        .title(":warning: error joining channel.")
                        .description("Please ensure I have the correct permissions.")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    }

    Ok(())
}
