use sha2::{Digest, Sha256};

use std::io::Write;
use std::path::PathBuf;

use crate::origin;

pub fn verify(path: &PathBuf) -> String {
  let mut file = std::fs::File::open(path).expect(&format!(
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

pub fn copy(new_file: &PathBuf, destiny_dir: &PathBuf) {
  std::fs::create_dir_all(&destiny_dir).expect(&format!(
    "Could not create the destiny dir: '{}'",
    destiny_dir.display()
  ));
  let destiny_file = origin::new(destiny_dir, new_file);
  std::fs::copy(new_file, destiny_file).expect(&format!(
    "Could not copy the eat file: '{}' to the destiny: '{}'",
    new_file.display(),
    destiny_dir.display()
  ));
}

pub fn append_lines(file_path: &PathBuf, lines: &str) {
  let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(!file_path.exists())
    .append(file_path.exists())
    .open(file_path)
    .expect(&format!(
      "Could not append lines because could not open the file: '{}'",
      file_path.display()
    ));
  writeln!(file, "{}", lines).expect(&format!(
    "Could not append lines because could not write the file: '{}'",
    file_path.display()
  ));
}
