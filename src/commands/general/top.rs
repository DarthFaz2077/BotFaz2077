use crate::models::bot::data::PgPoolContainer;
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, guild::Member},
};
use sqlx::query;
use tracing::warn;

struct User {
    member: Member,
    level: i32,
    current_xp: i32,
}

#[command]
#[description = "Show top people from this server."]
#[example("")]
#[only_in(guilds)]
async fn top(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();

    let result = query!(
        "SELECT users.user_id, users.level, users.current_xp
        FROM users_guilds
        INNER JOIN users ON users_guilds.user_id = users.user_id
        WHERE users_guilds.guild_id = $1
        ORDER BY users.total_xp DESC
        LIMIT 10",
        msg.guild_id.unwrap().0 as i64
    )
    .fetch_all(&pg_pool)
    .await?;

    let guild = msg.guild(ctx).await.unwrap();

    let mut users: Vec<User> = Vec::with_capacity(10);

    for record in result {
        match guild.member(ctx, record.user_id as u64).await {
            Ok(m) => {
                users.push(User {
                    member: m,
                    level: record.level,
                    current_xp: record.current_xp,
                });
            }
            Err(e) => warn!("{}", e),
        }
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Top");
                e.description("Top people from the server.");
                for (count, user) in users.iter().enumerate() {
                    e.field(
                        count + 1,
                        format!(
                            "{}, Level: {}, XP: {}/100",
                            user.member.user.tag(),
                            user.level,
                            user.current_xp
                        ),
                        false,
                    );
                }
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
