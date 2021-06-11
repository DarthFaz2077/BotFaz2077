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
    racing: Racing,
    survivor: Survivor,
    defilante: Defilante,
}

#[derive(Deserialize)]
struct Shaman {
    cheese: i64,
    saves_normal: i64,
    saves_hard: i64,
    saves_divine: i64,
}

#[derive(Deserialize)]
struct Normal {
    cheese: i64,
    first: i64,
    bootcamp: i64,
}

#[derive(Deserialize)]
struct Racing {
    rounds: i64,
    finished: i64,
    first: i64,
    podium: i64,
}

#[derive(Deserialize)]
struct Survivor {
    rounds: i64,
    killed: i64,
    shaman: i64,
    survivor: i64,
}

#[derive(Deserialize)]
struct Defilante {
    rounds: i64,
    finished: i64,
    points: i64,
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
                e.field(
                    response_json.name,
                    format!(
                        "• Position: {}
                        • Total members: {}
                        • Active members: {}",
                        response_json.position,
                        response_json.members.total,
                        response_json.members.active
                    ),
                    false,
                );
                e.field(
                    "Shaman",
                    format!(
                        "• Mice with cheese saved: {} / {} / {}
                        • Cheese personally gathered: {}",
                        response_json.stats.shaman.saves_normal,
                        response_json.stats.shaman.saves_hard,
                        response_json.stats.shaman.saves_divine,
                        response_json.stats.shaman.cheese
                    ),
                    false,
                );
                e.field(
                    "Mouse",
                    format!(
                        "• Cheese gathered first: {}
                        • Gathered cheese: {}
                        • Bootcamp: {}",
                        response_json.stats.normal.first,
                        response_json.stats.normal.cheese,
                        response_json.stats.normal.bootcamp
                    ),
                    false,
                );
                e.field(
                    "Racing",
                    format!(
                        "• Rounds played: {}
                        • Completed rounds: {}
                        • Number of podiums: {}
                        • Number of firsts: {}",
                        response_json.stats.racing.rounds,
                        response_json.stats.racing.finished,
                        response_json.stats.racing.podium,
                        response_json.stats.racing.first
                    ),
                    false,
                );
                e.field(
                    "Survivor",
                    format!(
                        "• Rounds played: {}
                        • Number of times Shaman: {}
                        • Killed mice: {}
                        • Rounds survived: {}",
                        response_json.stats.survivor.rounds,
                        response_json.stats.survivor.shaman,
                        response_json.stats.survivor.killed,
                        response_json.stats.survivor.survivor
                    ),
                    false,
                );
                e.field(
                    "Défilante",
                    format!(
                        "• Rounds played: {}
                        • Completed rounds: {}
                        • Points gathered: {}",
                        response_json.stats.defilante.rounds,
                        response_json.stats.defilante.finished,
                        response_json.stats.defilante.points
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

    Ok(())
}
