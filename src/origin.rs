use std::path::PathBuf;

pub fn new(on_dir: &PathBuf, from_file: &PathBuf) -> PathBuf {
  let mut new_name = String::from("main");
  if let Some(from_extension) = from_file.extension() {
    if let Some(from_extension) = from_extension.to_str() {
      new_name.push_str(".");
      new_name.push_str(from_extension);
    }
  }
  on_dir.join(new_name)
}

pub fn find(directory: &PathBuf) -> PathBuf {
  let entries = directory.read_dir().expect(&format!(
    "Could not read dir on find origin of: '{}'",
    directory.display()
  ));
  for entry in entries {
    if let Ok(entry) = entry {
      let file_name = entry.file_name();
      let file_name = file_name.to_str();
      if let Some(file_name) = file_name {
        if file_name.starts_with("main") && !file_name.ends_with(".meta") {
          return directory.join(file_name);
        }
      }
    }
  }
  panic!("Could not find the origin of: '{}'", directory.display());
}
