use std::env::var;

use dotenv::dotenv;
use poise::{ApplicationContext, CreateReply, Modal};
use serenity::all::{ChannelId, CreateEmbed, CreateMessage, User};

use crate::{Data, Error, ExpectError};

#[derive(Debug, Modal)]
struct ReportModal {
	reason: String,
}

#[poise::command(slash_command, context_menu_command = "Report User", ephemeral = true)]
pub async fn report(
	ctx: ApplicationContext<'_, Data, Error>,
	#[description = "Reported user"] user: User,
) -> Result<(), Error> {
	use poise::Modal as _;
	dotenv().ok();

	let data = ReportModal::execute(ctx).await?;

	if let Some(data) = data {
		let embed = CreateEmbed::default()
			.title("New Report")
			.description(format!(
				r#"```
{}
```
"#,
				data.reason
			))
			.field("Reported User", format!("<@{}>", user.id), true)
			.field("Reporter", format!("<@{}>", ctx.author().id), true)
			.color(0xd14821)
			.timestamp(serenity::model::Timestamp::now());

		dotenv().ok();
		let channel_id = var("REPORT_CHANNEL_ID")
			.expect_error(
				"Missing `REPORT_CHANNEL_ID` env var, please include this in your .env file",
			)
			.parse::<u64>()
			.expect_error("REPORT_CHANNEL_ID must be a valid u64 number");

		let mut message = CreateMessage::new().embed(embed);

		if let Ok(notification_role_id) = var("REPORT_NOTIFICATION_ROLE") {
			if let Ok(role_id) = notification_role_id.parse::<u64>() {
				message = message.content(format!("<@&{}>", role_id));
			}
		}

		ChannelId::new(channel_id)
			.send_message(ctx.serenity_context(), message)
			.await?;

		ctx.send(
			CreateReply::default()
				.content("Report submitted successfully! Thanks for helping out")
				.ephemeral(true),
		)
		.await?;
	}
	Ok(())
}
