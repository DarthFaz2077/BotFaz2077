use chrono::prelude::*;
use reqwest::Url;
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[derive(Deserialize)]
struct Root {
    list: Vec<List>,
}

#[derive(Deserialize)]
struct List {
    definition: String,
    permalink: String,
    example: String,
}
#[derive(Debug, Deserialize)]
struct Crypto {
    #[serde(default)]
    ticker: Ticker,
    #[serde(default)]
    timestamp: i64,
    success: bool,
    error: String,
}

#[derive(Debug, Deserialize, Default)]
struct Ticker {
    base: String,
    target: String,
    price: String,
    volume: String,
    change: String,
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn urban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let term = args.message();
    let request_url = Url::parse_with_params(
        "https://api.urbandictionary.com/v0/define?",
        &[("term", term)],
    )?;
    let response = reqwest::get(request_url).await?.json::<Root>().await?;

    if response.list.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Urban Dictionary");
                    e.description("No results found!");
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face())
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
                    e.title("Urban Dictionary");
                    e.url(response.list[0].permalink.to_string());
                    e.description(term);
                    e.field(
                        "Top definition:",
                        response.list[0].definition.to_string(),
                        false,
                    );
                    e.field("Example:", response.list[0].example.to_string(), false);
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face())
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

#[command]
async fn crypto(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let base = args.single::<String>().unwrap();
    let target = args.single::<String>().unwrap();
    let request_url = Url::parse(&format!(
        "https://api.cryptonator.com/api/ticker/{}-{}",
        base, target
    ))?;
    let response = reqwest::get(request_url).await?.json::<Crypto>().await?;

    if response.success == false {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Crypto Checker");
                    e.description(response.error);
                    e.footer(|f| {
                        f.text(format!("Requested by {}", msg.author.tag()));
                        f.icon_url(msg.author.face())
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
                            f.icon_url(msg.author.face())
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
                            f.icon_url(msg.author.face())
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
