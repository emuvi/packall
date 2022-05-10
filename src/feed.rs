use crossbeam;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use std::fs;
use std::path::PathBuf;
use unicase::UniCase;

use crate::data::Body;

pub fn start(body: &Body, path: &str) {
  println!("Feeding from: '{}'...", path);
  crossbeam::scope(|scope| {
    let (sender, recv) = unbounded::<PathBuf>();
    scope.spawn(|_| {
      let path = PathBuf::from(path);
      if path.is_dir() {
        feed_dir(body, path, &sender);
      } else {
        feed_file(body, path, &sender);
      }
      drop(sender);
    });
    for _ in 0..body.head.speed {
      let receiver = recv.clone();
      scope.spawn(move |_| {
        for received in receiver {
          eat_file(body, received);
        }
      });
    }
  })
  .expect("Feeding processes panicked!");
}

fn feed_dir(body: &Body, dir: PathBuf, sender: &Sender<PathBuf>) {
  println!("Feeding folder: '{}'...", dir.display());
  let entries: Vec<_> = dir
    .read_dir()
    .expect(&format!(
      "Could not read feeding folder: '{}'",
      dir.display()
    ))
    .collect();
  for entry in &entries {
    if let Ok(entry) = entry.as_ref() {
      let path = entry.path();
      if path.is_dir() {
        feed_dir(body, path, sender);
      }
    }
  }
  let mut has_files = false;
  for entry in &entries {
    if let Ok(entry) = entry.as_ref() {
      let path = entry.path();
      if !path.is_dir() {
        has_files = true;
        feed_file(body, path, sender);
      }
    }
  }
  if !has_files && body.head.clean {
    if fs::remove_dir(&dir).is_ok() {
      println!("Cleaned the folder: '{}'", dir.display());
    }
  }
}

fn feed_file(body: &Body, file: PathBuf, sender: &Sender<PathBuf>) {
  println!("Feeding file: '{}'", file.display());
  if file.exists() {
    let mut allowed = false;
    if body.head.allow_extensions.is_empty() {
      allowed = true;
    } else {
      if let Some(extension) = file.extension() {
        if let Some(extension) = extension.to_str() {
          for permitted in &body.head.allow_extensions {
            if UniCase::new(extension) == UniCase::new(permitted) {
              allowed = true;
              break;
            }
          }
        }
      }
    }
    if allowed {
      let err_msg = format!("Could not send to be eaten this file: '{}'", file.display());
      sender.send(file).expect(&err_msg);
    } else {
      println!(
        "Feeding failed because was not allowed this file: '{}' ",
        file.display()
      );
    }
  } else {
    println!(
      "Feeding failed because does not exists the file: '{}'",
      file.display()
    );
  }
}

pub fn eat_file(body: &Body, path: PathBuf) {
  println!("Eating file: '{}'", path.display());
  let verifier = crate::utils::get_verifier(&path);
  println!("Verified file: '{}' as: '{}'", path.display(), verifier);
  let first = &verifier[0..3];
  let second = &verifier[3..6];
  let destiny = body.home.tree.join(first).join(second).join(&verifier);
  if !destiny.exists() {
    println!("Copying the file: '{}' as: '{}'", path.display(), verifier);
    crate::utils::copy_file(&path, &destiny);
  } else {
    println!(
      "We already have the file: '{}' as: '{}'",
      path.display(),
      verifier
    );
  }
  crate::utils::add_meta_data(&path, &destiny);
  if body.head.clean {
    if std::fs::remove_file(&path).is_ok() {
      println!("Cleaned the file: '{}'", path.display());
    } else {
      println!("Could not clean the file: '{}'", path.display());
    }
    if let Some(parent) = &path.parent() {
      if std::fs::remove_dir(parent).is_ok() {
        println!("Cleaned the folder: '{}'", parent.display());
      }
    }
  }
  println!(
    "Successfully eaten file: '{}' by: '{}'",
    path.display(),
    verifier
  );
}
