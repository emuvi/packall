mod body;
mod clip;
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
	let clip = clip::run();
	let body = String::from(clip.value_of("body").expect("You must inform the body."));
	let speed_str = clip.value_of("speed").expect("You must pass a speed.");
	let speed = speed_str.parse().expect("You must pass a valid speed.");
	let clean = clip.is_present("clean");
	let brain = body::Head::new(body, speed, clean);
	brain.start();
	if let Some(path) = clip.value_of("feed") {
		brain.feed(path);
	}
	if clip.is_present("digest") {
		brain.digest();
	}
	if let Some(words) = clip.value_of("search") {
		brain.search(words);
	}
	if let Some(path) = clip.value_of("lend") {
		brain.lend(path);
	}
	if let Some(path) = clip.value_of("give") {
		brain.give(path);
	}
	if clip.is_present("junk") {
		brain.junk();
	}
	if clip.is_present("open") {
		brain.open();
	}
	println!("PackAll finished execution!");
}
