use crate::models::bot::data::PgPoolContainer;
use chrono::Utc;
use rand::random;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use sqlx::query;

#[command]
#[description = "Gamble your coins."]
#[example("amount")]
#[example("all")]
async fn gamble(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Gamble");
                    e.description("Send an amount of coins to gamble, please!");
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
    let pg_pool = data.get::<PgPoolContainer>().cloned().unwrap();

    let message = args.single::<String>().unwrap();

    let amount = if message == "all" {
        let result = query!(
            "SELECT coins FROM users WHERE user_id = $1",
            msg.author.id.0 as i64
        )
        .fetch_one(&pg_pool)
        .await?;
        result.coins
    } else {
        message.parse::<i64>().unwrap()
    };

    let result = query!(
        "SELECT coins FROM users WHERE user_id = $1",
        msg.author.id.0 as i64
    )
    .fetch_one(&pg_pool)
    .await?;

    if amount > result.coins {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Gamble");
                    e.description("You don't have enough coins.");
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

    if random() {
        query!(
            "UPDATE users SET coins = coins + $1 WHERE user_id = $2",
            amount,
            msg.author.id.0 as i64
        )
        .execute(&pg_pool)
        .await?;

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Gamble");
                    e.description(format!("You won {} coins.", amount));
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
        query!(
            "UPDATE users SET coins = coins - $1 WHERE user_id = $2",
            amount,
            msg.author.id.0 as i64
        )
        .execute(&pg_pool)
        .await?;

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Gamble");
                    e.description(format!("You lost {} coins.", amount));
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

    Ok(())
}
