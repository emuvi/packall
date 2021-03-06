mod clip;

fn main() {
  let args = clip::parse();
  let body = if args.is_present("body") {
    String::from(
      args
        .value_of("body")
        .expect("Problem on read body argument."),
    )
  } else {
    let body_err_msg = "You let me as an errant soul. You must give me a body, as an argument -b / --body or with the environment variable PACKALL_BODY";
    let body_env = std::env::var_os("PACKALL_BODY").expect(body_err_msg);
    let body_env = body_env.to_str().expect(body_err_msg);
    format!("{}", body_env)
  };
  let speed_str = args.value_of("speed").expect("You must pass a speed.");
  let speed = speed_str.parse().expect("You must pass a valid speed.");
  let clean = args.is_present("clean");
  println!("Packall starting...");
  println!("Body: '{}'", body);
  println!("Speed: '{}'", speed);
  println!("Clean: '{}'", clean);
  let body = packall::data::Body::new(body, speed, clean);
  if let Some(path) = args.value_of("feed") {
    packall::feed::start(&body, path);
  }
  if args.is_present("digest") {
    packall::digest::start(&body);
  }
  if let Some(words) = args.value_of("search") {
    packall::search::start(&body, words);
  }
  if let Some(path) = args.value_of("lend") {
    packall::lend::start(&body, path);
  }
  if let Some(path) = args.value_of("give") {
    packall::give::start(&body, path);
  }
  if args.is_present("junk") {
    packall::junk::start(&body);
  }
  if args.is_present("open") {
    packall::open::start(&body);
  }
  println!("#------------------------------------------------------#");
  body.logs.print();
  println!("#------------------------------------------------------#");
  println!("PackAll finished execution!");
}
