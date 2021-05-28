use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	let msg = msg.channel_id.send_message(&ctx.http, |m| {
		m.embed(|e| {
			e.title("Pong");
			e.color(Colour::DARK_GREEN);
			e
		});
		m
	}).await;

	if let Err(why) = msg {
		println!("Error sending message: {:?}", why);
	}

	Ok(())
}
