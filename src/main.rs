mod body;
mod cli;
mod digest;
mod feed;
mod give;
mod junk;
mod lend;
mod meta;
mod open;
mod search;
mod utils;

fn main() {
	let cli = cli::run();
	let body = String::from(cli.value_of("body").expect("You must inform the body."));
	let speed_str = cli.value_of("speed").expect("You must pass a speed.");
	let speed = speed_str.parse().expect("You must pass a valid speed.");
	let clean = cli.is_present("clean");
	let brain = body::Head::new(body, speed, clean);
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
	println!("PackAll finished execution!");
}
