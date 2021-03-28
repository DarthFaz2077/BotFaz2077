use crate::models::bot::data::PgPoolContainer;
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query;

#[command]
#[description = "Get your daily coins."]
#[example("")]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();
    let date = Utc::now().naive_utc().date();

    let result = query!(
        "SELECT daily_date FROM users WHERE user_id = $1",
        msg.author.id.0 as i64
    )
    .fetch_one(&pg_pool)
    .await?;

    if result.daily_date >= Some(date) {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Daily");
                    e.description("Come back tomorrow.");
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

    query!(
        "UPDATE users SET coins = coins + 100, daily_date = $1 WHERE user_id = $2",
        date,
        msg.author.id.0 as i64
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Daily");
                e.description("You got 100 coins.");
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
