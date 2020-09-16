use serde::Deserialize;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[derive(Deserialize)]
struct Response {
    image: String,
}

#[command]
async fn poke(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, "Mention someone, please.").await?;
        return Ok(());
    }

    let mentioned_user = args.single::<UserId>().unwrap().to_user(ctx).await?;
    let request_url = "http://api.nekos.fun:8080/api/poke";
    let response = reqwest::get(request_url).await?.json::<Response>().await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} pokes {}",
                    msg.author.tag(),
                    mentioned_user.tag()
                ));
                e.image(response.image);
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn feed(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, "Mention someone, please.").await?;
        return Ok(());
    }

    let mentioned_user = args.single::<UserId>().unwrap().to_user(ctx).await?;
    let request_url = "http://api.nekos.fun:8080/api/feed";
    let response = reqwest::get(request_url).await?.json::<Response>().await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} feeds {}",
                    msg.author.tag(),
                    mentioned_user.tag()
                ));
                e.image(response.image);
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn pat(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id.say(ctx, "Mention someone, please.").await?;
        return Ok(());
    }

    let mentioned_user = args.single::<UserId>().unwrap().to_user(ctx).await?;
    let request_url = "http://api.nekos.fun:8080/api/pat";
    let response = reqwest::get(request_url).await?.json::<Response>().await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} pats {}",
                    msg.author.tag(),
                    mentioned_user.tag()
                ));
                e.image(response.image);
                e
            })
        })
        .await?;

    Ok(())
}
