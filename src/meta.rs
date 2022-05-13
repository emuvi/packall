use chrono::offset::Utc;
use chrono::DateTime;

use std::path::PathBuf;

pub fn add_all_meta_data(of_file: &PathBuf, destiny_dir: &PathBuf) {
  if let Some(file_name) = of_file.file_name() {
    if let Some(file_name) = file_name.to_str() {
      add_file_name_on(&destiny_dir, file_name);
    }
  }
  if let Some(parent) = of_file.parent() {
    add_file_tree_on(&destiny_dir, &format!("{}", parent.display()));
  }
  if let Ok(meta) = of_file.metadata() {
    if let Ok(created) = meta.created() {
      let created: DateTime<Utc> = created.into();
      let created = created.format("%Y-%m-%dT%H:%M:%S").to_string();
      add_file_created_on(&destiny_dir, &created);
    }
    if let Ok(modified) = meta.modified() {
      let modified: DateTime<Utc> = modified.into();
      let modified = modified.format("%Y-%m-%dT%H:%M:%S").to_string();
      add_file_modified_on(&destiny_dir, &modified);
    }
    if let Ok(accessed) = meta.accessed() {
      let accessed: DateTime<Utc> = accessed.into();
      let accessed = accessed.format("%Y-%m-%dT%H:%M:%S").to_string();
      add_file_accessed_on(&destiny_dir, &accessed);
    }
  }
  if let Some(old_main_meta) = find_main_meta(of_file) {
    import_main_meta_on(&destiny_dir, &old_main_meta);
  }
}

pub fn add_file_name_on(destiny_dir: &PathBuf, file_name: &str) {
  add_main_meta_on(destiny_dir, "file_name", file_name);
}

pub fn add_file_tree_on(destiny_dir: &PathBuf, file_tree: &str) {
  add_main_meta_on(destiny_dir, "file_tree", file_tree);
}

pub fn add_file_created_on(destiny_dir: &PathBuf, file_tree: &str) {
  add_main_meta_on(destiny_dir, "file_created", file_tree);
}

pub fn add_file_modified_on(destiny_dir: &PathBuf, file_tree: &str) {
  add_main_meta_on(destiny_dir, "file_modified", file_tree);
}

pub fn add_file_accessed_on(destiny_dir: &PathBuf, file_tree: &str) {
  add_main_meta_on(destiny_dir, "file_accessed", file_tree);
}

pub fn import_main_meta_on(destiny_dir: &PathBuf, from_main_meta_file: &PathBuf) {
  let to_main_meta_file = get_main_meta_of(destiny_dir);
  let read_main_meta_data =
    std::fs::read_to_string(from_main_meta_file).expect(&format!(
      "Could not read old main meta from: {}",
      from_main_meta_file.display()
    ));
  crate::files::append_lines(&to_main_meta_file, read_main_meta_data.trim());
}

pub fn add_main_meta_on(destiny_dir: &PathBuf, key: &str, value: &str) {
  if key.is_empty() || value.is_empty() {
    return;
  }
  let main_meta_path = get_main_meta_of(destiny_dir);
  let line = format!("{} = {}", key, value);
  crate::files::append_lines(&main_meta_path, &line);
}

pub fn get_main_meta_of(destiny_dir: &PathBuf) -> PathBuf {
  return destiny_dir.join("main.meta");
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
