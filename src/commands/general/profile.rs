use crate::models::{bot::data::PgPoolContainer, database::user::User};
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query_as;

#[command]
#[description = "Show your own profile."]
#[example("")]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();

    let result = query_as!(
        User,
        "SELECT * FROM users WHERE user_id = $1",
        msg.author.id.0 as i64
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Profile");
                e.description(format!("{}'s profile.", msg.author.tag()));
                e.field("XP:", format!("{}/100", result.current_xp), false);
                e.field("Level:", result.level, false);
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
