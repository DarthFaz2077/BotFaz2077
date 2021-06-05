use crate::models::bot::data::ReqwestClientContainer;
use chrono::Utc;
use reqwest::Url;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use urlencoding::encode;

#[derive(Deserialize)]
struct ResponseJson {
    //id: i64,
    name: String,
    members: Members,
    stats: Stats,
    position: i64,
}

#[derive(Deserialize)]
struct Members {
    total: i64,
    active: i64,
}

#[derive(Deserialize)]
struct Stats {
    shaman: Shaman,
    normal: Normal,
    //survivor: Survivor,
    //racing: Racing,
    //defilante: Defilante,
    //score: Score,
}

#[derive(Deserialize)]
struct Shaman {
    //cheese: i64,
    saves_normal: i64,
    saves_hard: i64,
    saves_divine: i64,
}

#[derive(Deserialize)]
struct Normal {
    //rounds: i64,
    cheese: i64,
    first: i64,
    bootcamp: i64,
}

#[derive(Deserialize)]
struct Survivor {
    //rounds: i64,
//killed: i64,
//shaman: i64,
//survivor: i64,
}

#[derive(Deserialize)]
struct Racing {
    //rounds: i64,
//finished: i64,
//first: i64,
//podium: i64,
}

#[derive(Deserialize)]
struct Defilante {
    //rounds: i64,
//finished: i64,
//points: i64,
}

#[derive(Deserialize)]
struct Score {
    //stats: i64,
//shaman: i64,
//survivor: i64,
//racing: i64,
//defilante: i64,
//overall: i64,
}

#[command]
#[description = "Check stats of tribe on Transformice."]
#[example("name")]
async fn tribe(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Transformice Tribe Stats");
                    e.description("Send a name to search for, please!");
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
    let name = encode(&args.rest());
    let request_url = Url::parse(&format!("https://cheese.formice.com/api/tribes/{}", name))?;
    let response = reqwest_client.get(request_url).send().await?;

    if !response.status().is_success() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Transformice Tribe Stats");
                    e.description("Something happened!");
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

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Transformice Tribe Stats");
                e.description(&response_json.name);
                e.field("Total members:", &response_json.members.total, false);
                e.field("Active members:", &response_json.members.active, false);
                e.field(
                    "Normal saves:",
                    &response_json.stats.shaman.saves_normal,
                    false,
                );
                e.field(
                    "Hardmode Saves:",
                    &response_json.stats.shaman.saves_hard,
                    false,
                );
                e.field(
                    "Divine Saves:",
                    &response_json.stats.shaman.saves_divine,
                    false,
                );
                e.field(
                    "Cheese gathered first:",
                    &response_json.stats.normal.first,
                    false,
                );
                e.field(
                    "Gathered cheese:",
                    &response_json.stats.normal.cheese,
                    false,
                );
                e.field("Bootcamp:", &response_json.stats.normal.bootcamp, false);
                e.field("Position:", &response_json.position, false);
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
