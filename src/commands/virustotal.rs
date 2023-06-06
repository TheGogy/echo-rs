use std::env;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use virustotal::*;

#[command]
#[aliases("virustotal", "vt")]
async fn scan(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let apikey =
        env::var("VIRUSTOTAL_API_KEY").expect("Set your VIRUSTOTAL_API_KEY environment variable!");
    let url = args.single::<String>().unwrap();

    let vt = VtClient::new(&apikey);
    let res = vt.scan_url(&url);

    let report: UrlReportResponse = vt.report_url(&res.scan_id.unwrap());

    println!("\n\n{:?}\n\n", report);
    let p = report.positives.unwrap();
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.colour(if p < 4 {
                    0xa6e3a1 // safe
                } else if p < 10 {
                    0xf9e2af // suspicious
                } else {
                    0xf38ba8 // malicious
                })
                .title(":shield: URL Scanned")
                .url(report.permalink.unwrap())
                .description(format!(
                    "--> `{}`\n\nHits --> {:?}/{:?}",
                    url,
                    p,
                    report.total.unwrap()
                ))
                .timestamp(Timestamp::now())
            })
        })
        .await?;

    Ok(())
}
