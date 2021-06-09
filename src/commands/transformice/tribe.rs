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
                e.description(&response_json.name);
                e.field(
                    "[Members] Total members:",
                    &response_json.members.total,
                    false,
                );
                e.field(
                    "[Members] Active members:",
                    &response_json.members.active,
                    false,
                );
                e.field(
                    "[Shaman] Normal saves:",
                    &response_json.stats.shaman.saves_normal,
                    false,
                );
                e.field(
                    "[Shaman] Hardmode Saves:",
                    &response_json.stats.shaman.saves_hard,
                    false,
                );
                e.field(
                    "[Shaman] Divine Saves:",
                    &response_json.stats.shaman.saves_divine,
                    false,
                );
                e.field(
                    "[Mouse] Cheese gathered first:",
                    &response_json.stats.normal.first,
                    false,
                );
                e.field(
                    "[Mouse] Gathered cheese:",
                    &response_json.stats.normal.cheese,
                    false,
                );
                e.field(
                    "[Mouse] Bootcamp:",
                    &response_json.stats.normal.bootcamp,
                    false,
                );
                e.field(
                    "[Racing] Rounds played:",
                    &response_json.stats.racing.rounds,
                    false,
                );
                e.field(
                    "[Racing] Completed rounds:",
                    &response_json.stats.racing.finished,
                    false,
                );
                e.field(
                    "[Racing] Number of podiums:",
                    &response_json.stats.racing.podium,
                    false,
                );
                e.field(
                    "[Racing] Number of firsts:",
                    &response_json.stats.racing.first,
                    false,
                );
                e.field(
                    "[Survivor] Rounds played:",
                    &response_json.stats.survivor.rounds,
                    false,
                );
                e.field(
                    "[Survivor] Number of times Shaman:",
                    &response_json.stats.survivor.shaman,
                    false,
                );
                e.field(
                    "[Survivor] Killed mice:",
                    &response_json.stats.survivor.killed,
                    false,
                );
                e.field(
                    "[Survivor] Rounds survived:",
                    &response_json.stats.survivor.survivor,
                    false,
                );
                e.field(
                    "[Défilante] Rounds played:",
                    &response_json.stats.defilante.rounds,
                    false,
                );
                e.field(
                    "[Défilante] Completed rounds:",
                    &response_json.stats.defilante.finished,
                    false,
                );
                e.field(
                    "[Défilante] Points gathered:",
                    &response_json.stats.defilante.points,
                    false,
                );
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
