use std::path::PathBuf;

use crate::utils;

pub fn get_main_meta_path(from_item_origin: &PathBuf) -> PathBuf {
  let item_dir = from_item_origin.parent().expect(&format!(
    "Could not get the file parent from the item origin: '{}'",
    from_item_origin.display()
  ));
  let file_steam = from_item_origin.file_stem().expect(&format!(
    "Could not get the file steam from the item origin: '{}'",
    from_item_origin.display()
  ));
  let file_steam = file_steam.to_str();
  let file_steam = file_steam.expect(&format!(
    "Could not get the file steam str from the item origin: '{}'",
    from_item_origin.display()
  ));
  return item_dir.join(format!("{}.main.meta", file_steam));
}

pub fn add_main_meta(item_origin: &PathBuf, key: &str, value: &str) {
  if key.is_empty() {
    panic!("Something went wrong! I've got an empty key to add in the meta.");
  }
  if value.is_empty() {
    return;
  }
  let main_meta_path = get_main_meta_path(item_origin);
  let line = format!("{} = {}", key, value);
  utils::append_lines_on_file(&main_meta_path, &line);
}

pub fn add_file_name(item_origin: &PathBuf, file_name: &str) {
  add_main_meta(item_origin, "file_name", file_name);
}

pub fn add_file_tree(item_origin: &PathBuf, file_tree: &str) {
  add_main_meta(item_origin, "file_tree", file_tree);
}

pub fn import_main_meta(to_item_origin: &PathBuf, from_main_meta: &PathBuf) {
  let new_main_meta_path = get_main_meta_path(to_item_origin);
  let old_main_meta_data = std::fs::read_to_string(from_main_meta).expect(&format!(
    "Could not read old main meta from: {}",
    from_main_meta.display()
  ));
  utils::append_lines_on_file(&new_main_meta_path, old_main_meta_data.trim());
}
