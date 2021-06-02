use crate::models::bot::data::PgPoolContainer;
use chrono::{Duration, Utc};
use humantime::format_duration;
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
    let timestamp = Utc::now().naive_utc();

    let result = query!(
        "SELECT next_daily FROM users WHERE user_id = $1",
        msg.author.id.0 as i64
    )
    .fetch_one(&pg_pool)
    .await?;

    if result.next_daily >= Some(timestamp) {
        let difference = result.next_daily.unwrap() - timestamp;
        let formatted_difference = format_duration(difference.to_std().unwrap());

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Daily");
                    e.description(format!("Come back in {}.", formatted_difference));
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

    let next_daily = timestamp + Duration::days(1);

    query!(
        "UPDATE users SET coins = coins + 100, next_daily = $1 WHERE user_id = $2",
        next_daily,
        msg.author.id.0 as i64
    )
    .execute(&pg_pool)
    .await?;

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
