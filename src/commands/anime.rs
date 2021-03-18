use crate::structures::client_data::ReqwestClient;
use chrono::Utc;
use reqwest::Url;
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[derive(Deserialize)]
struct Response {
    image: String,
}

#[command]
async fn poke(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Poke");
                    e.description("Mention someone, please!");
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

    let response = fetch_nekos_gif(ctx, "poke").await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Poke");
                e.description(format!(
                    "{} pokes {}d",
                    msg.author.tag(),
                    msg.mentions[0].tag()
                ));
                e.image(response.image);
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

    Ok(())
}

#[command]
async fn feed(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Poke");
                    e.description("Mention someone, please!");
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

    let response = fetch_nekos_gif(ctx, "feed").await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Feed");
                e.description(format!(
                    "{} feeds {}",
                    msg.author.tag(),
                    msg.mentions[0].tag()
                ));
                e.image(response.image);
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

    Ok(())
}

#[command]
async fn pat(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Poke");
                    e.description("Mention someone, please!");
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

    let response = fetch_nekos_gif(ctx, "pat").await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Pat");
                e.description(format!(
                    "{} pats {}",
                    msg.author.tag(),
                    msg.mentions[0].tag()
                ));
                e.image(response.image);
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

    Ok(())
}

#[command]
async fn baka(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Poke");
                    e.description("Mention someone, please!");
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

    let response = fetch_nekos_gif(ctx, "baka").await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Baka");
                e.description(format!(
                    "{} calls {} a baka",
                    msg.author.tag(),
                    msg.mentions[0].tag()
                ));
                e.image(response.image);
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

    Ok(())
}

async fn fetch_nekos_gif(ctx: &Context, tag: &str) -> Result<Response, anyhow::Error> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().cloned().unwrap();
    let request_url = Url::parse(&format!("http://api.nekos.fun:8080/api/{}", tag))?;
    let response = reqwest_client
        .get(request_url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(response)
}
