use serde_json::Value;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Head {
	pub body: PathBuf,
	pub speed: usize,
	pub clean: bool,
	pub allow_extensions: Vec<String>,
}

impl Head {
	pub fn new(body: String, speed: usize, clean: bool) -> Head {
		if let Ok(body_meta) = fs::metadata(&body) {
			if body_meta.is_file() {
				panic!("The body path must be a directory.");
			}
		} else {
			if fs::create_dir_all(&body).is_err() {
				panic!("Could not create the body directory.");
			}
		}
		let mut allow_extensions: Vec<String> = vec![];
		let body_path = Path::new(&body).to_owned();
		let body_cfg_path = body_path.join("(meta)").join("body.json");
		if body_cfg_path.exists() {
			let body_cfg_json = fs::read_to_string(body_cfg_path)
				.expect("The body config could not be read.");
			let body_cfg_value: Value = serde_json::from_str(&body_cfg_json)
				.expect("The body config could not be parse.");
			match &body_cfg_value["allow_extensions"] {
				Value::Null => {}
				Value::Array(values) => {
					for value in values {
						match value {
							Value::String(value_str) => {
								allow_extensions.push(String::from(value_str));
							}
							_ => {
								println!(
									"Wrong value type por body config: 'allow_extensions' item."
								);
							}
						}
					}
				}
				_ => {
					println!("Wrong value type por body config: 'allow_extensions'.");
				}
			}
		}
		Head {
			body: body_path,
			speed,
			clean,
			allow_extensions,
		}
	}
}
