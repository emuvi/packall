use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn add_main_meta(item_origin: &PathBuf, key: &str, value: &str) {
    if key.is_empty() {
        panic!("Something went wrong! I've got an empty key to add in the meta.");
    }
    if value.is_empty() {
        return;
    }
    let item_dir = item_origin.parent().expect(&format!(
        "Could not get the file parent from the item origin: '{}'",
        item_origin.display()
    ));
    let file_steam = item_origin.file_stem().expect(&format!(
        "Could not get the file steam from the item origin: '{}'",
        item_origin.display()
    ));
    let file_steam = file_steam.to_str();
    let file_steam = file_steam.expect(&format!(
        "Could not get the file steam str from the item origin: '{}'",
        item_origin.display()
    ));
    let main_meta_path = item_dir.join(format!("{}.main.meta", file_steam));
    let mut file = OpenOptions::new()
        .write(true)
        .create(!main_meta_path.exists())
        .append(main_meta_path.exists())
        .open(&main_meta_path)
        .expect(&format!(
            "Could not open main meta file on: '{}'",
            main_meta_path.display()
        ));
    writeln!(file, "{} = {}", key, value).expect(&format!(
        "Could not write main meta file on: '{}'",
        main_meta_path.display()
    ));
}

pub fn add_file_name(item_origin: &PathBuf, file_name: &str) {
    add_main_meta(item_origin, "file_name", file_name);
}

pub fn add_file_tree(item_origin: &PathBuf, file_tree: &str) {
    add_main_meta(item_origin, "file_tree", file_tree);
}
