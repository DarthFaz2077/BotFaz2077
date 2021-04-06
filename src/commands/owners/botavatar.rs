use crate::models::bot::data::ReqwestClientContainer;
use chrono::Utc;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Change or remove the bot's avatar."]
#[example("remove")]
#[example("image.png")]
#[example("https://example.com/image.png")]
#[owners_only]
async fn botavatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() && msg.attachments.is_empty() {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Bot Avatar");
                    e.description("You must send an image to be set as the avatar or \"remove\" to remove the avatar!");
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

    let mut user = ctx.cache.current_user().await;

    if args.message() == "remove" {
        user.edit(ctx, |p| {
            p.avatar(None);

            p
        })
        .await?;

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Bot Avatar");
                    e.description("Removed the bot's avatar!");
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

    let url = if !args.message().is_empty() {
        args.message()
    } else {
        &msg.attachments[0].url
    };

    let response = reqwest_client.get(url).send().await?;
    let headers = response.headers().to_owned();
    if let Some(content_type) = headers.get("Content-Type") {
        let content_type_str = content_type.to_str()?;
        let image = response.bytes().await?;
        let base64 = base64::encode(image);
        user.edit(ctx, |p| {
            p.avatar(Some(&format!(
                "data:{};base64,{}",
                content_type_str, base64
            )));

            p
        })
        .await?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Bot Avatar");
                e.description("Changed the bot's avatar.");
                e.image(user.face());
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
