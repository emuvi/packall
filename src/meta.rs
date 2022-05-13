use std::path::PathBuf;

use crate::utils;

pub fn get_main_meta_of(destiny_dir: &PathBuf) -> PathBuf {
  return destiny_dir.join("main.meta");
}

pub fn add_main_meta_on(destiny_dir: &PathBuf, key: &str, value: &str) {
  if key.is_empty() || value.is_empty() {
    return;
  }
  let main_meta_path = get_main_meta_of(destiny_dir);
  let line = format!("{} = {}", key, value);
  utils::append_lines_on_file(&main_meta_path, &line);
}

pub fn add_file_name_on(destiny_dir: &PathBuf, file_name: &str) {
  add_main_meta_on(destiny_dir, "file_name", file_name);
}

pub fn add_file_tree_on(destiny_dir: &PathBuf, file_tree: &str) {
  add_main_meta_on(destiny_dir, "file_tree", file_tree);
}

pub fn import_main_meta_on(destiny_dir: &PathBuf, from_main_meta_file: &PathBuf) {
  let to_main_meta_file = get_main_meta_of(destiny_dir);
  let read_main_meta_data =
    std::fs::read_to_string(from_main_meta_file).expect(&format!(
      "Could not read old main meta from: {}",
      from_main_meta_file.display()
    ));
  utils::append_lines_on_file(&to_main_meta_file, read_main_meta_data.trim());
}
