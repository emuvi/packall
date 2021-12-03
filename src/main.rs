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
	let args = clip::parse();
	let body: String;
	if args.is_present("body") {
		body = String::from(args.value_of("body").expect("Problem on read body argument."));
	} else {
		let body_err_msg = "You let me as an errant soul. You must give me a body, as an argument -b / --body or environment variable PACKALL_BODY";
		let body_env = std::env::var_os("PACKALL_BODY").expect(body_err_msg);
		let body_env = body_env.to_str().expect(body_err_msg);
		body = format!("{}", body_env);
	}
	let speed_str = args.value_of("speed").expect("You must pass a speed.");
	let speed = speed_str.parse().expect("You must pass a valid speed.");
	let clean = args.is_present("clean");
	println!("Packall starting...");
	println!("Body: '{}'", body);
	println!("Speed: '{}'", speed);
	println!("Clean: '{}'", clean);
	let brain = body::Head::new(body, speed, clean);
	if let Some(path) = args.value_of("feed") {
		brain.feed(path);
	}
	if args.is_present("digest") {
		brain.digest();
	}
	if let Some(words) = args.value_of("search") {
		brain.search(words);
	}
	if let Some(path) = args.value_of("lend") {
		brain.lend(path);
	}
	if let Some(path) = args.value_of("give") {
		brain.give(path);
	}
	if args.is_present("junk") {
		brain.junk();
	}
	if args.is_present("open") {
		brain.open();
	}
	println!("PackAll finished execution!");
}
