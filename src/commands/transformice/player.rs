use crate::models::bot::data::ReqwestClientContainer;
use chrono::Utc;
use reqwest::Url;
use resvg::render;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use std::{collections::HashMap, u8};
use tiny_skia::Pixmap;
use urlencoding::encode;
use usvg::{Options, Tree};

#[derive(Deserialize)]
struct ResponseJson {
    id: i64,
    name: String,
    title: String,
    tribe: Option<Tribe>,
    soulmate: Option<Soulmate>,
    shop: Shop,
    stats: Stats,
    position: i64,
}

#[derive(Deserialize)]
struct Tribe {
    name: String,
}

#[derive(Deserialize)]
struct Soulmate {
    name: String,
}

#[derive(Deserialize)]
struct Shop {
    look: String,
    mouse_color: i64,
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
#[description = "Check stats of player on Transformice."]
#[example("Name#number")]
async fn player(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Transformice Player Stats");
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
    let name = encode(&args.single::<String>().unwrap());
    let request_url = Url::parse(&format!("https://cheese.formice.com/api/players/{}", name))?;
    let response = reqwest_client.get(request_url).send().await?;

    if !response.status().is_success() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Transformice Player Stats");
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

    let title_number = format!("T_{}", response_json.title);
    let request_url = Url::parse_with_params(
        "https://cheese.formice.com/api/translation/en?",
        &[("field", title_number.clone())],
    )?;
    let response = reqwest_client.get(request_url).send().await?;
    let response_hashmap: HashMap<String, String> = serde_json::from_str(&response.text().await?)?;

    let request_url = Url::parse(&format!(
        "https://cheese.formice.com/api/dressroom/mouse/{};{:X}",
        response_json.shop.look, response_json.shop.mouse_color
    ))?;
    let response = reqwest_client.get(request_url).send().await?;
    let response_svg = response.text().await?;

    let mut pixmap: Pixmap;

    {
        let opt = Options::default();
        let rtree = Tree::from_str(&response_svg, &opt).unwrap();
        pixmap = Pixmap::new(512, 512).unwrap();
        render(&rtree, usvg::FitTo::Height(512), pixmap.as_mut()).unwrap();
    }

    let outfit = pixmap.encode_png().unwrap();

    let full_id = response_json.id.to_string();
    let len = full_id.len();
    let partial_id = &full_id[len - 4..];

    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file((outfit.as_slice(), "outfit.png"));
            m.embed(|e| {
                e.title("Transformice Player Stats");
                e.field(
                    response_json.name,
                    format!(
                        "• Position: {}
                        • Title: {}
                        • Soumate: {}
                        • Tribe: {}",
                        response_json.position,
                        response_hashmap.get(&title_number).unwrap(),
                        match response_json.soulmate {
                            Some(soulmate) => soulmate.name,
                            None => "No soulmate".to_string(),
                        },
                        match response_json.tribe {
                            Some(tribe) => tribe.name,
                            None => "No tribe".to_string(),
                        }
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
                e.thumbnail(format!(
                    "https://avatars.atelier801.com/{}/{}.jpg",
                    partial_id, full_id
                ));
                e.image("attachment://outfit.png");
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
