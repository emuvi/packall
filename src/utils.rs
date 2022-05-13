use sha2::{Digest, Sha256};

use std::io::Write;
use std::path::PathBuf;

pub fn get_verifier(path: &PathBuf) -> String {
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

pub fn copy_file(new_file: &PathBuf, destiny_dir: &PathBuf) {
  std::fs::create_dir_all(&destiny_dir).expect(&format!(
    "Could not create the destiny dir: '{}'",
    destiny_dir.display()
  ));
  let destiny_file = new_origin(destiny_dir, new_file);
  std::fs::copy(new_file, destiny_file).expect(&format!(
    "Could not copy the eat file: '{}' to the destiny: '{}'",
    new_file.display(),
    destiny_dir.display()
  ));
}

pub fn find_main_meta(item_file: &PathBuf) -> Option<PathBuf> {
  let parent = match item_file.parent() {
    Some(parent) => parent,
    None => return None,
  };
  let main_meta_path = parent.join("main.meta");
  if main_meta_path.exists() {
    return Some(main_meta_path);
  }
  None
}

pub fn add_meta_data(of_file: &PathBuf, destiny_dir: &PathBuf) {
  if let Some(file_name) = of_file.file_name() {
    if let Some(file_name) = file_name.to_str() {
      crate::meta::add_file_name_on(&destiny_dir, file_name);
    }
  }
  if let Some(parent) = of_file.parent() {
    crate::meta::add_file_tree_on(&destiny_dir, &format!("{}", parent.display()));
  }
  if let Some(old_main_meta) = find_main_meta(of_file) {
    crate::meta::import_main_meta_on(&destiny_dir, &old_main_meta);
  }
}

pub fn find_origin(item_dir: &PathBuf) -> PathBuf {
  let entries = item_dir.read_dir().expect(&format!(
    "Could not read dir on find origin of: '{}'",
    item_dir.display()
  ));
  for entry in entries {
    if let Ok(entry) = entry {
      let file_name = entry.file_name();
      let file_name = file_name.to_str();
      if let Some(file_name) = file_name {
        if file_name.starts_with("main") && !file_name.ends_with(".meta") {
          return item_dir.join(file_name);
        }
      }
    }
  }
  panic!("Could not find the origin of: '{}'", item_dir.display());
}

pub fn new_origin(item_dir: &PathBuf, from_path: &PathBuf) -> PathBuf {
  let mut new_name = String::from("main");
  if let Some(from_extension) = from_path.extension() {
    if let Some(from_extension) = from_extension.to_str() {
      new_name.push_str(".");
      new_name.push_str(from_extension);
    }
  }
  item_dir.join(new_name)
}

pub fn append_lines_on_file(file_path: &PathBuf, lines: &str) {
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
