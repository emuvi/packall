use std::collections::HashMap;
use std::sync::RwLock;

pub struct Logger {
  pub summary: RwLock<HashMap<&'static str, usize>>,
}

impl Logger {
  pub fn new() -> Logger {
    Logger {
      summary: RwLock::new(HashMap::new()),
    }
  }

  pub fn sum_one(&self, of_kind: &'static str) {
    let mut lock = self.summary.write().unwrap();
    let mut count = if let Some(count) = lock.get(of_kind) {
      *count
    } else {
      0
    };
    count += 1;
    lock.insert(of_kind, count);
  }

  pub fn print(&self) {
    println!("Summary: ");
    let lock = self.summary.read().unwrap();
    for (kind, count) in lock.iter() {
      println!("{}: {}", kind, count);
    }
  }
}
