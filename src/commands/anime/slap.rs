use crate::models::bot::data::ReqwestClientContainer;
use chrono::Utc;
use reqwest::Url;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[derive(Deserialize)]
struct Response {
    image: String,
}

#[command]
async fn slap(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Slap");
                    e.description("Mention someone, please!");
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
    let request_url = Url::parse("http://api.nekos.fun:8080/api/slap")?;
    let response = reqwest_client
        .get(request_url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Slap");
                e.description(format!(
                    "{} slaps {}.",
                    msg.author.tag(),
                    msg.mentions[0].tag()
                ));
                e.image(response.image);
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

    Ok(())
}
