use lazy_static::lazy_static;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DieRoll {
	value: u32,
	keep:  bool,
	uuid:  String,
}

impl DieRoll {
	pub fn new(value: u32, keep: bool, uuid: String) -> Self {
		DieRoll { value, keep, uuid }
	}
}

#[derive(Clone, Debug)]
pub struct RollSet {
	dice: Vec<DieRoll>,
	total: u32,
	roll_string: String,
}

impl RollSet {
	pub fn new(dice: Vec<DieRoll>, roll_string: String) -> Self {
		RollSet { dice,  total: 0, roll_string }
	}
}


fn get_match_string(captures: &regex::Captures, name: &str) -> String {
	let capture = captures.name(name);

	let result: String;
	if capture.is_some() {
		let value = capture.unwrap().as_str();
		result = value.to_string();
	}
	else {
		result = "".to_string();
	}

	result
}

fn get_match_value(captures: &regex::Captures, name: &str, default: u32) -> u32 {
	let capture = captures.name(name);

	let result: u32;
	if capture.is_some() {
		let capture = capture.unwrap().as_str();
		result = capture.parse().unwrap();
	}
	else {
		result = default;
	}

	result
}

pub fn parse(command: &String) -> Vec<RollSet> {
	lazy_static! {
		// A line-by-line break down of the regex:
		//
		// (?x)
		//
		// This turns on free-spacing, which ignores whitespace between regular
		// expression tokens to make them easier to read.
		//
		// ^(?P<raw_string>
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
		// (?P<rolls>\d+)?
		//
		// Now we have the number of dice to include as part of this roll. This
		// is captured in "rolls" and is optional..
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
		static ref RE: Regex = Regex::new(r"(?x)
			^(?P<raw_string>
				(?:(?P<count>\d+)@)?
				(?P<roll_string>
					(?P<rolls>\d+)?
					d(?P<sides>\d+)
					(?:(?P<kd>[kd][hl])(?P<kdcount>\d+)?)?
					(?:(?P<mod>[+\-*/])(?P<mod_value>\d+))?
				)
			)
			$
		").unwrap();
	}

	let mut results = Vec::new();
	if RE.is_match(command.as_str()) {
		let captures = RE.captures(command.as_str()).unwrap();

		let roll_string = get_match_string(&captures, "roll_string");

		let count = get_match_value(&captures, "count",  1);
		let rolls = get_match_value(&captures, "rolls",  1);
		let sides = get_match_value(&captures, "sides", 20);

		results = roll(count, rolls, sides, roll_string);

		let kd = get_match_string(&captures, "kd");
		let kdcount = get_match_value(&captures, "kdcount", 1) as i32;

		if kd != "" {
			for set in results.iter_mut() {
				let mut sorted = set.dice.to_vec();
				sorted.sort();

				let mut num_to_keep = sorted.len() as i32;
				if kd == "kl" {
					num_to_keep = kdcount;
				}
				else if kd == "dl" {
					sorted.reverse();
					num_to_keep = sorted.len() as i32 - kdcount;
				}
				else if kd == "dh" {
					num_to_keep = sorted.len() as i32 - kdcount;
				}
				else if kd == "kh" {
					sorted.reverse();
					num_to_keep = kdcount;
				}

				let mut keep_drop = HashMap::new();
				let mut total: u32 = 0;

				for roll in sorted.iter() {
					if num_to_keep > 0 {
						keep_drop.insert(roll.uuid.to_string(), true);
						total += roll.value;
					}
					else {
						keep_drop.insert(roll.uuid.to_string(), false);
					}

					num_to_keep = num_to_keep - 1;
				}

				for roll in set.dice.iter_mut() {
					let keep = keep_drop.get(&roll.uuid).unwrap();
					roll.keep = *keep;
				}

				set.total = total;

				for roll in set.dice.iter() {
					println!("roll: {}, keep: {}", roll.value, roll.keep);
				}
			}
		}
	}

	results
}

pub fn roll(count: u32, rolls: u32, sides: u32, roll_string: String) -> Vec<RollSet> {
	let mut rng = rand::thread_rng();
	let mut results = Vec::new();

	for _ in 1..count + 1 {
		let mut set: Vec<DieRoll> = Vec::new();

		for _ in 1..rolls + 1 {
			let uuid = Uuid::new_v4().to_hyphenated().to_string();

			let value = rng.gen_range(1..sides + 1);
			set.push(DieRoll::new(value, true, uuid));
		}

		results.push(RollSet::new(set, roll_string.to_string()));
	}

	results
}
