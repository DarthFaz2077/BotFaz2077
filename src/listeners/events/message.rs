use crate::models::bot::data::MongoDBContainer;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;
use serenity::{client::Context, model::channel::Message};

pub async fn message(ctx: &Context, new_message: Message) {
    if new_message.author.bot {
        return;
    }

    let data = ctx.data.read().await;
    let mongodb_client = data.get::<MongoDBContainer>().cloned().unwrap();
    let database_name = ctx
        .cache
        .current_user_field(|user| user.name.to_string())
        .await;
    let collection = mongodb_client.database(&database_name).collection("users");

    collection
        .update_one(
            doc! { "_id": new_message.author.id.to_string() },
            doc! { "$inc": { "xp": 1 } },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await
        .unwrap();

    let result = collection
        .find_one(doc! { "_id": new_message.author.id.to_string() }, None)
        .await
        .unwrap()
        .unwrap();

    let xp = result.get_i32("xp").unwrap_or(0);
    let level = result.get_i32("level").unwrap_or(0);

    if xp >= 100 {
        collection
            .update_one(
                doc! { "_id": new_message.author.id.to_string() },
                doc! { "$inc": { "xp": -100, "level": 1 } },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await
            .unwrap();

        new_message
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Level up");
                    e.description(format!(
                        "{} leveled up to level {}.",
                        new_message.author.tag(),
                        level + 1
                    ));
                    e.timestamp(&Utc::now());

                    e
                });

                m
            })
            .await
            .unwrap();
    }
}
