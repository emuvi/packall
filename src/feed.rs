use crossbeam;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use unicase::UniCase;

use crate::body;
use crate::meta;
use crate::utils;

impl body::Head {
	pub fn feed(&self, path: &str) {
		println!("Feeding from: '{}'", path);
		crossbeam::scope(|scope| {
			let (s, r) = unbounded::<PathBuf>();
			scope.spawn(|_| {
				let path = Path::new(path);
				if path.is_dir() {
					self.feed_dir(path.to_owned(), &s);
				} else {
					self.feed_file(path.to_owned(), &s);
				}
				drop(s);
			});
			for _ in 0..self.speed {
				let receiver = r.clone();
				scope.spawn(move |_| {
					for received in receiver {
						self.eat_file(received);
					}
				});
			}
		})
		.expect("Feeding processes panicked!");
	}

	fn feed_dir(&self, dir: PathBuf, sender: &Sender<PathBuf>) {
		println!("Feeding folder: '{}'", dir.display());
		let entries: Vec<_> = dir
			.read_dir()
			.expect(&format!(
				"Could not read feeding folder: '{}'",
				dir.display()
			))
			.collect();
		for entry in &entries {
			if let Ok(entry) = entry.as_ref() {
				let path = entry.path();
				if path.is_dir() {
					self.feed_dir(path, sender);
				}
			}
		}
		let mut has_files = false;
		for entry in &entries {
			if let Ok(entry) = entry.as_ref() {
				let path = entry.path();
				if !path.is_dir() {
					has_files = true;
					self.feed_file(path, sender);
				}
			}
		}
		if !has_files && self.clean {
			if fs::remove_dir(&dir).is_ok() {
				println!("Cleaned the folder: '{}'", dir.display());
			}
		}
	}

	fn feed_file(&self, file: PathBuf, sender: &Sender<PathBuf>) {
		println!("Feeding file: '{}'", file.display());
		if file.exists() {
			let mut allowed = false;
			if self.allow_extensions.is_empty() {
				allowed = true;
			} else {
				if let Some(extension) = file.extension() {
					if let Some(extension) = extension.to_str() {
						for permitted in &self.allow_extensions {
							if UniCase::new(extension) == UniCase::new(permitted) {
								allowed = true;
								break;
							}
						}
					}
				}
			}
			if allowed {
				let err_msg = format!("Could not send to be eaten this file: '{}'", file.display());
				sender.send(file).expect(&err_msg);
			} else {
				println!(
					"Feeding failed because was not allowed this file: '{}' ",
					file.display()
				);
			}
		} else {
			println!(
				"Feeding failed because does not exists the file: '{}'",
				file.display()
			);
		}
	}

	fn eat_file(&self, path: PathBuf) {
		println!("Eating file: '{}'", path.display());
		let verifier = get_verifier(&path);
		println!("Verified file: '{}' as: '{}'", path.display(), verifier);
		let first = &verifier[0..3];
		let second = &verifier[3..6];
		let destiny = self.body.join(first).join(second).join(&verifier);
		if !destiny.exists() {
			println!("Copying the file: '{}' as: '{}'", path.display(), verifier);
			copy_file(&path, &destiny);
		} else {
			println!(
				"We already have the file: '{}' as: '{}'",
				path.display(),
				verifier
			);
		}
		add_meta_data(&path, &destiny);
		if self.clean {
			if let Some(parent) = &path.parent() {
				if fs::remove_file(&path).is_ok() {
					println!("Cleaned the file: '{}'", path.display());
					if fs::remove_dir(parent).is_ok() {
						println!("Cleaned the folder: '{}'", parent.display());
					}
				} else {
					println!("Could not clean the file: '{}'", path.display());
				}
			}
		}
		println!(
			"Successfully eaten file: '{}' by: '{}'",
			path.display(),
			verifier
		);

		fn copy_file(path: &PathBuf, destiny: &PathBuf) {
			fs::create_dir_all(&destiny).expect(&format!(
				"Could not create the destiny dir: '{}'",
				destiny.display()
			));
			let destiny_file = utils::new_origin(destiny, path);
			fs::copy(path, destiny_file).expect(&format!(
				"Could not copy the eat file: '{}' to the destiny: '{}'",
				path.display(),
				destiny.display()
			));
		}

		fn add_meta_data(path: &PathBuf, destiny: &PathBuf) {
			let origin = utils::find_origin(&destiny);
			if let Some(file_name) = path.file_name() {
				if let Some(file_name) = file_name.to_str() {
					meta::add_file_name(&origin, file_name);
				}
			}
			if let Some(parent) = path.parent() {
				meta::add_file_tree(&origin, &format!("{}", parent.display()));
			}
			if let Some(old_main_meta) = utils::find_main_meta(path) {
				meta::import_main_meta(&origin, &old_main_meta);
			}
		}
	}
}

fn get_verifier(path: &PathBuf) -> String {
	let mut file = fs::File::open(path).expect(&format!(
		"Could not open to get the verifier of this file: '{}'",
		path.display()
	));
	let mut sha256 = Sha256::new();
	std::io::copy(&mut file, &mut sha256).expect(&format!(
		"Could not read to get the verifier of this file: '{}'",
		path.display()
	));
	format!("{:x}", sha256.finalize())
}
