use sha2::{Digest, Sha256};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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

pub fn copy_file(path: &PathBuf, destiny: &PathBuf) {
  std::fs::create_dir_all(&destiny).expect(&format!(
    "Could not create the destiny dir: '{}'",
    destiny.display()
  ));
  let destiny_file = new_origin(destiny, path);
  std::fs::copy(path, destiny_file).expect(&format!(
    "Could not copy the eat file: '{}' to the destiny: '{}'",
    path.display(),
    destiny.display()
  ));
}

pub fn find_main_meta(item_file: &PathBuf) -> Option<PathBuf> {
  let steam = match item_file.file_stem() {
    Some(steam) => steam,
    None => return None,
  };
  let steam = match steam.to_str() {
    Some(steam) => steam,
    None => return None,
  };
  let parent = match item_file.parent() {
    Some(parent) => parent,
    None => return None,
  };
  let entries = match parent.read_dir() {
    Ok(entries) => entries,
    Err(_) => return None,
  };
  let main_meta_name = format!("{}.main.meta", steam);
  for entry in entries {
    let entry = match entry {
      Ok(entry) => entry,
      Err(_) => continue,
    };
    let file_name = match entry.file_name().into_string() {
      Ok(file_name) => file_name,
      Err(_) => continue,
    };
    if file_name == main_meta_name {
      return Some(parent.join(main_meta_name));
    }
  }
  return None;
}

pub fn add_meta_data(path: &PathBuf, destiny: &PathBuf) {
  let origin = find_origin(&destiny);
  if let Some(file_name) = path.file_name() {
    if let Some(file_name) = file_name.to_str() {
      crate::meta::add_file_name(&origin, file_name);
    }
  }
  if let Some(parent) = path.parent() {
    crate::meta::add_file_tree(&origin, &format!("{}", parent.display()));
  }
  if let Some(old_main_meta) = find_main_meta(path) {
    crate::meta::import_main_meta(&origin, &old_main_meta);
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
        if file_name.starts_with("org-") && !file_name.ends_with(".meta") {
          return item_dir.join(file_name);
        }
      }
    }
  }
  panic!("Could not find the origin of: '{}'", item_dir.display());
}

pub fn new_origin(item_dir: &PathBuf, from_path: &PathBuf) -> PathBuf {
  let mut new_name = String::from("org-");
  let new_token: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(18)
    .map(char::from)
    .collect();
  new_name.push_str(&new_token.to_lowercase());
  let mut extension = String::new();
  if let Some(from_extension) = from_path.extension() {
    if let Some(from_extension) = from_extension.to_str() {
      extension.push_str(".");
      extension.push_str(from_extension);
    }
  }
  new_name.push_str(&extension);
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
