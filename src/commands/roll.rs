use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;
// use tracing::info;

use crate::dice::parse;

#[command]
#[aliases("r")]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
	lazy_static! {
		static ref RE: Regex = Regex::new(
			r"(?x)
			^!(?:r|roll)
			\s+
			(?P<command_body>[^:.]+)
			(?:\s*:\s*(?P<description>[\w\s]+)?)?
			$"
		)
		.unwrap();
	}

	println!("{:?}", msg);

	let msg_content = msg.content.as_str();
	println!("{:?}", msg_content);

	if RE.is_match(msg_content) {
		let captures = RE.captures(msg_content).unwrap();
		println!("{:?}", captures);

		let mut command_body = String::from(
			captures.name("command_body").unwrap().as_str()
		);
		command_body = String::from(command_body.trim());

		// Get a sensible response title, either using the desciption provided
		// or a default value
		//
		let mut title: String;
		let description = captures.name("description");

		if description.is_some() {
			title = String::from(description.unwrap().as_str());
			title = title.trim().to_string();
		} else {
			title = "Roll".to_string();
		}

		println!("{:?}", title);

		let results = parse(&command_body);

		println!("{:?}", results);
		println!("{}", msg.author.name);

		let msg = msg
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| {
					e.title(&title);
					e.color(Colour::DARK_GREEN);
					e.description(&command_body);
					e
				});
				m
			})
			.await;

		if let Err(why) = msg {
			println!("Error sending message: {:?}", why);
		}
	}
	else {
		let msg = msg
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| {
					e.title("Failed to Parse Roll");
					e.color(Colour::DARK_RED);
					e
				});
				m
			})
			.await;

		if let Err(why) = msg {
			println!("Error sending message: {:?}", why);
		}
	}

	Ok(())
}
