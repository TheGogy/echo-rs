use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::model::Timestamp;
use serenity::prelude::*;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e
            .colour(0x89dceb)
            .thumbnail("https://images.unsplash.com/photo-1574169208507-84376144848b?ixlib=rb-4.0.3")
            .title("**- -【 Ｈｅｌｐ 】- -**")
            .description("Hi! I'm Echo. A simple rust discord but made with Serenity! Prefix is `~`.")
            .fields(vec![
                ("help", "Displays this help menu", true),
            ])
            .footer(|f| f.text("Made by Gogy"))
            .timestamp(Timestamp::now())
        )
    }).await?;

    Ok(())
}