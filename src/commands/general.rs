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
                    e.description(term);
                    e.field(
                        "Top definition:",
                        response.list[0].definition.to_string(),
                        false,
                    );
                    e.field("Example:", response.list[0].example.to_string(), false);
                    e.url(response.list[0].permalink.to_string());
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
