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
    #[serde(rename = "list")]
    definitions: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
    #[serde(rename = "definition")]
    description: String,
    permalink: String,
    example: String,
}

#[command]
#[description = "Check a definition in Urban Dictionary."]
#[example("word")]
async fn urban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Urban Dictionary");
                    e.description("Send a term to search for, please!");
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
    let term = args.message();
    let request_url = Url::parse_with_params(
        "https://api.urbandictionary.com/v0/define?",
        &[("term", term)],
    )?;
    let response = reqwest_client
        .get(request_url)
        .send()
        .await?
        .json::<Response>()
        .await?;

    if response.definitions.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Urban Dictionary");
                    e.description("No results found!");
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
                    e.title("Urban Dictionary");
                    e.url(response.definitions[0].permalink.to_string());
                    e.description(term);
                    e.field(
                        "Top definition:",
                        response.definitions[0].description.to_string(),
                        false,
                    );
                    e.field(
                        "Example:",
                        response.definitions[0].example.to_string(),
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
