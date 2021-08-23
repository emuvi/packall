extern crate sha2;
use crate::head;

use sha2::{Sha256, Digest};
use std::path::Path;
use std::path::PathBuf;

impl head::Head<'_> {
	pub fn feed(&self, path: &str) {
		println!("Feeding from: {}", path);
		let path = Path::new(path);
		if path.is_dir() {
			self.feed_dir(path.to_owned());
		} else {
			self.feed_file(path.to_owned());
		}
	}

	fn feed_dir(&self, dir: PathBuf) {
		let entries = dir.read_dir().unwrap();
		for entry in entries {
			let entry = entry.unwrap();
			let path = entry.path();
			if path.is_dir() {
				self.feed_dir(path);
			} else {
				self.feed_file(path);
			}
		}
	}

	fn feed_file(&self, file: PathBuf) {
		println!("Feeding file: {}", file.display());
		println!("File verifier: {}", self.get_verifier(&file));
	}

	fn get_verifier(&self, file: &PathBuf) -> String {
		let mut file = std::fs::File::open(file).unwrap();
		let mut sha256 = Sha256::new();
		std::io::copy(&mut file, &mut sha256).unwrap();
		format!("{:x}", sha256.finalize())
	}
}
