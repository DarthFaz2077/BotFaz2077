use serde::Deserialize;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[derive(Deserialize)]
struct Poke {
    image: String,
}

#[command]
async fn poke(ctx: &Context, msg: &Message) -> CommandResult {
    let request_url = format!(
        "http://api.nekos.fun:8080/api/{endpoint}",
        endpoint = "poke"
    );

    let resp = reqwest::get(&request_url).await?.json::<Poke>().await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} pokes {}",
                    msg.author.name, msg.mentions[0].name
                ));
                e.image(resp.image);

                e
            })
        })
        .await?;

    Ok(())
}
