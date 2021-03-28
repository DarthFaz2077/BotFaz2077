use crate::models::bot::data::PgPoolContainer;
use chrono::Utc;
use serenity::{client::Context, model::channel::Message};
use sqlx::query;

pub async fn message(ctx: &Context, new_message: Message) {
    if new_message.author.bot {
        return;
    }

    let data = ctx.data.read().await;
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();

    query!(
        "INSERT INTO users (user_id) VALUES ($1) ON CONFLICT DO NOTHING",
        new_message.author.id.0 as i64
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    query!(
        "UPDATE users SET total_xp = total_xp + 1, current_xp = current_xp + 1 WHERE user_id = $1",
        new_message.author.id.0 as i64
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    let result = query!(
        "SELECT current_xp, level FROM users WHERE user_id = $1",
        new_message.author.id.0 as i64
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    if result.current_xp >= 100 {
        query!(
            "UPDATE users SET current_xp = 0, level = level + 1 WHERE user_id = $1",
            new_message.author.id.0 as i64
        )
        .execute(&pg_pool)
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
                        result.level + 1
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
