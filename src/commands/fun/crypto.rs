use crate::models::bot::data::ReqwestClientContainer;
use chrono::Utc;
use reqwest::Url;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[derive(Deserialize)]
struct Response {
    #[serde(default)]
    ticker: Ticker,
    #[serde(default)]
    success: bool,
    error: String,
}

#[derive(Deserialize, Default)]
struct Ticker {
    base: String,
    target: String,
    price: String,
    volume: String,
    change: String,
}

#[command]
async fn crypto(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description("Send a pair to search for, please!");
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face());

                        f
                    });
                    e.timestamp(&Utc::now());

                    e
                });

                m
            })
            .await?;

        return Ok(());
    }

    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClientContainer>().cloned().unwrap();
    let base = args.single::<String>().unwrap();
    let target = args.single::<String>().unwrap();
    let request_url = Url::parse(&format!(
        "https://api.cryptonator.com/api/ticker/{}-{}",
        base, target
    ))?;
    let response = reqwest_client
        .get(request_url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    if response.success == false {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description(response.error);
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face());

                        f
                    });
                    e.timestamp(&Utc::now());

                    e
                });

                m
            })
            .await?;
    } else {
        if response.ticker.volume.is_empty() {
            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Crypto Checker");
                        e.description(response.ticker.base);
                        e.field(
                            "Price:",
                            format!("{} {}", response.ticker.price, response.ticker.target),
                            false,
                        );
                        e.field(
                            "1h change:",
                            format!("{} {}", response.ticker.change, response.ticker.target),
                            false,
                        );
                        e.footer(|f| {
                            f.text(format!("Requested by {}", msg.author.tag()));
                            f.icon_url(msg.author.face());

                            f
                        });
                        e.timestamp(&Utc::now());

                        e
                    });

                    m
                })
                .await?;
        } else {
            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Crypto Checker");
                        e.description(&response.ticker.base);
                        e.field(
                            "Price:",
                            format!("{} {}", response.ticker.price, response.ticker.target),
                            false,
                        );
                        e.field(
                            "24h volume:",
                            format!("{} {}", response.ticker.volume, response.ticker.base),
                            false,
                        );
                        e.field(
                            "1h change:",
                            format!("{} {}", response.ticker.change, response.ticker.target),
                            false,
                        );
                        e.footer(|f| {
                            f.text(format!("Requested by {}", msg.author.tag()));
                            f.icon_url(msg.author.face());

                            f
                        });
                        e.timestamp(&Utc::now());

                        e
                    });

                    m
                })
                .await?;
        }
    }

    Ok(())
}