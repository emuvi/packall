use std::fs;

mod cli;
mod digest;
mod feed;
mod give;
mod head;
mod junk;
mod lend;
mod open;
mod search;

fn main() {
	let cli = cli::run();
	let body = cli.value_of("body").expect("You must inform the body.");
	let speed_str = cli.value_of("speed").expect("You must pass a speed.");
	let speed = speed_str.parse().expect("You must pass a valid speed.");
	let clean = cli.is_present("clean");
	if let Ok(body_meta) = fs::metadata(body) {
		if body_meta.is_file() {
			panic!("The body path must be a directory.");
		}
	} else {
		if fs::create_dir_all(body).is_err() {
			panic!("Could not create the body directory.");
		}
	}
	let brain = head::Head { body, speed, clean };
	brain.start();
	if let Some(path) = cli.value_of("feed") {
		brain.feed(path);
	}
	if cli.is_present("digest") {
		brain.digest();
	}
	if let Some(words) = cli.value_of("search") {
		brain.search(words);
	}
	if let Some(path) = cli.value_of("lend") {
		brain.lend(path);
	}
	if let Some(path) = cli.value_of("give") {
		brain.give(path);
	}
	if cli.is_present("junk") {
		brain.junk();
	}
	if cli.is_present("open") {
		brain.open();
	}
}
