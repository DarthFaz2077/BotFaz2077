use crate::models::bot::data::MongoDBContainer;
use chrono::Utc;
use mongodb::bson::doc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Show your own profile."]
#[example("")]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let mongodb_client = data.get::<MongoDBContainer>().cloned().unwrap();
    let database_name = ctx
        .cache
        .current_user_field(|user| user.name.to_string())
        .await;
    let collection = mongodb_client.database(&database_name).collection("users");

    let result = collection
        .find_one(doc! { "_id": msg.author.id.to_string() }, None)
        .await
        .unwrap()
        .unwrap();

    let xp = result.get_i32("xp").unwrap_or(0);
    let level = result.get_i32("level").unwrap_or(0);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Profile");
                e.description(format!("{}'s profile.", msg.author.tag()));
                e.field("XP:", format!("{}/100", xp), false);
                e.field("Level:", level, false);
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
