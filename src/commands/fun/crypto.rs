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
struct ResponseJson {
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
#[description = "Check price of cryptocurrency."]
#[example("TAG1 TAG2")]
async fn crypto(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description("Send a pair to search for, please!");
                    e.footer(|f| {
                        f.text(format!("Requested by {}.", msg.author.tag()));
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
    let response = reqwest_client.get(request_url).send().await?;

    if !response.status().is_success() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description("There was a problem getting the results!");
                    e.footer(|f| {
                        f.text(format!("Requested by {}.", msg.author.tag()));
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

    let response_json = response.json::<ResponseJson>().await?;

    if !response_json.success {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description(response_json.error);
                    e.footer(|f| {
                        f.text(format!("Requested by {}.", msg.author.tag()));
                        f.icon_url(msg.author.face());

                        f
                    });
                    e.timestamp(&Utc::now());

                    e
                });

                m
            })
            .await?;
    } else if response_json.ticker.volume.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description(response_json.ticker.base);
                    e.field(
                        "Price:",
                        format!(
                            "{} {}",
                            response_json.ticker.price, response_json.ticker.target
                        ),
                        false,
                    );
                    e.field(
                        "1h change:",
                        format!(
                            "{} {}",
                            response_json.ticker.change, response_json.ticker.target
                        ),
                        false,
                    );
                    e.footer(|f| {
                        f.text(format!("Requested by {}.", msg.author.tag()));
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
                    e.description(&response_json.ticker.base);
                    e.field(
                        "Price:",
                        format!(
                            "{} {}",
                            response_json.ticker.price, response_json.ticker.target
                        ),
                        false,
                    );
                    e.field(
                        "24h volume:",
                        format!(
                            "{} {}",
                            response_json.ticker.volume, response_json.ticker.base
                        ),
                        false,
                    );
                    e.field(
                        "1h change:",
                        format!(
                            "{} {}",
                            response_json.ticker.change, response_json.ticker.target
                        ),
                        false,
                    );
                    e.footer(|f| {
                        f.text(format!("Requested by {}.", msg.author.tag()));
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

    Ok(())
}
