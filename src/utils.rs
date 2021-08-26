use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

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

pub fn put_suffix(on_file_path: &PathBuf, suffix: &str) {
  // TODO
}