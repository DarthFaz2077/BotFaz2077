use crate::models::bot::data::PgPoolContainer;
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use sqlx::query;

#[command]
#[description = "Show the profiles of mentioned people, or your own."]
#[example("")]
#[example("@mention")]
#[example("@mention1 @mention2")]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();

    if msg.mentions.is_empty() {
        let result = query!(
            "SELECT level, current_xp, coins FROM users WHERE user_id = $1",
            msg.author.id.0 as i64
        )
        .fetch_one(&pg_pool)
        .await?;

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Profile");
                    e.description(format!("{}'s profile.", msg.author.tag()));
                    e.field("XP:", format!("{}/100", result.current_xp), false);
                    e.field("Level:", result.level, false);
                    e.field("Coins:", result.coins, false);
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
    } else {
        for user in &msg.mentions {
            match query!(
                "SELECT level, current_xp, coins FROM users WHERE user_id = $1",
                user.id.0 as i64
            )
            .fetch_one(&pg_pool)
            .await
            {
                Ok(result) => {
                    msg.channel_id
                        .send_message(ctx, |m| {
                            m.embed(|e| {
                                e.title("Profile");
                                e.description(format!("{}'s profile.", user.tag()));
                                e.field("XP:", format!("{}/100", result.current_xp), false);
                                e.field("Level:", result.level, false);
                                e.field("Coins:", result.coins, false);
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
                }
                Err(_) => {
                    msg.channel_id
                        .send_message(ctx, |m| {
                            m.embed(|e| {
                                e.title("Profile");
                                e.description(format!("{} doesn't have a profile.", user.tag()));
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
                }
            }
        }
    }

    Ok(())
}
