use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;
use std::io::Write;

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
