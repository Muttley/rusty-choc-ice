use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[aliases("r")]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
	lazy_static! {
		// A line-by-line break down of the regex:
		//
		// (?x)
		//
		// This turns on free-spacing, which ignores whitespace between regular
		// expression tokens to make them easier to read.
		//
		// ^!(?:r|roll)\s+
		//
		// Ignore the command or its alias.
		//
		// (?P<raw_string>
		//
		// Captures the entire matching roll string to the named capture
		// "raw_string" to use later in the response.
		//
		// (?:(?P<count>\d+)@)?
		//
		// An optional number followed by a "@" means that number of the
		// following rolls have been requested. The number required is captured
		// in "count".
		//
		// (?P<multiplier>\d+)?
		//
		// Now we have the number of dice to include as part of this roll. This
		// is captured in "multiplier" and is optional..
		//
		// d(?P<sides>\d+)
		//
		// The number of sides on the die being requested is captured "sides".
		//
		// (?:(?P<kd>[kd][hl])(?P<kdcount>\d+))?
		//
		// Allow "kh" (keep highest), "kl" (keep lowest), "dh" (drop highest)
		// and "dh" (drop highest) as part of the roll, along with an optional
		// count of how many rolls to drop/keep.
		//
		// (?:(?P<mod>[+\-*\/])(?P<mod_value>\d+))?
		//
		// Captures any mathematic operation that is requested to be done on
		// the result of the rolls into "mod" and "mod_value".
		//
		// )
		//
		// End of the "raw_string" capture.
		//
		// (?:\s+(?P<description>.+)?)?
		//
		// An optional description for the roll that will be used as the title
		// of the response if it exists.
		//
		static ref RE: Regex = Regex::new(r"(?x)
			^!(?:r|roll)\s+
			(?P<raw_string>
				(?:(?P<count>\d+)@)?
				(?P<multiplier>\d+)?
				d(?P<sides>\d+)
				(?:(?P<kd>[kd][hl])(?P<kdcount>\d+)?)?
				(?:(?P<mod>[+\-*/])(?P<mod_value>\d+))?
			)
			(?:\s+(?P<description>.+)?)?
		").unwrap();
	}

	let msg_content = msg.content.as_str();
	println!("{:?}", msg_content);

	if RE.is_match(msg_content) {
		let captures = RE.captures(msg.content.as_str()).unwrap();

		println!("{:?}", captures);
		let msg = msg.channel_id.send_message(&ctx.http, |m| {
			m.embed(|e| {
				e.title("Roll");
				e.color(Colour::DARK_GREEN);
				e.description(&captures["raw_string"]);
				e
			});
			m
		}).await;

		if let Err(why) = msg {
			println!("Error sending message: {:?}", why);
		}
	}
	else {
		let msg = msg.channel_id.send_message(&ctx.http, |m| {
			m.embed(|e| {
				e.title("Failed to Parse Roll");
				e.color(Colour::DARK_RED);
				e
			});
			m
		}).await;

		if let Err(why) = msg {
			println!("Error sending message: {:?}", why);
		}
	}

	Ok(())
}
