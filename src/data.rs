use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;

use crate::logs::Logger;

pub struct Body {
  pub head: Head,
  pub home: Home,
  pub logs: Logger,
}

pub struct Home {
  pub meta: PathBuf,
  pub pack: PathBuf,
  pub root: PathBuf,
}

pub struct Head {
  pub speed: usize,
  pub clean: bool,
  pub allow_extensions: Vec<String>,
}

impl Body {
  pub fn new(root: String, speed: usize, clean: bool) -> Body {
    let root = Path::new(&root);
    if root.is_file() {
      panic!("The body path must be a directory.");
    } else {
      if std::fs::create_dir_all(&root).is_err() {
        panic!("Could not create the body directory.");
      }
    }
    let mut allow_extensions: Vec<String> = vec![];
    let root = root.to_owned();
    let pack = root.join("(pack)");
    if std::fs::create_dir_all(&pack).is_err() {
      panic!("Could not create the body pack directory.");
    }
    let meta = pack.join("(meta)");
    if std::fs::create_dir_all(&meta).is_err() {
      panic!("Could not create the pack meta directory.");
    }
    let meta_cfgs_path = meta.join("cfgs.json");
    if meta_cfgs_path.exists() {
      let meta_cfgs_json = std::fs::read_to_string(meta_cfgs_path)
        .expect("The meta configs could not be read.");
      let meta_cfgs_value: Value = serde_json::from_str(&meta_cfgs_json)
        .expect("The meta configs could not be parsed.");
      match &meta_cfgs_value["allow_extensions"] {
        Value::Null => {}
        Value::Array(values) => {
          for value in values {
            match value {
              Value::String(value_str) => {
                allow_extensions.push(String::from(value_str));
              }
              _ => {
                eprintln!("Wrong value type por body config: 'allow_extensions' item.");
              }
            }
          }
        }
        _ => {
          eprintln!("Wrong value type por body config: 'allow_extensions'.");
        }
      }
    }
    Body {
      head: Head {
        speed,
        clean,
        allow_extensions,
      },
      home: Home { root, pack, meta },
      logs: Logger::new(),
    }
  }
}
