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
#[description = "Laugh."]
#[example("")]
#[only_in(guilds)]
async fn laugh(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClientContainer>().cloned().unwrap();
    let request_url = Url::parse("http://api.nekos.fun:8080/api/laugh")?;
    let response = reqwest_client
        .get(request_url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Laugh");
                e.description(format!("{} laughs.", msg.author.tag()));
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
