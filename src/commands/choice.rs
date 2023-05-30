use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

#[command]
#[aliases("random", "choose")]
async fn choice(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let max = args.single::<u32>().unwrap_or(10);

    if max <= 1 {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xf38ba8)
                        .title(":warning: Number must be at least 2.")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;

        return Ok(());
    }

    let mut rng = StdRng::from_entropy();
    let result = rng.gen_range(1..max);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(0xa6e3a1)
                    .title(format!("Random digit --> {}", result))
                    .description(format!("Range --> 1 - {}", max))
                    .timestamp(Timestamp::now())
                    .footer(|f| f.text(if result == 69 { "Nice" } else { " - " }))
            })
        })
        .await?;

    Ok(())
}
