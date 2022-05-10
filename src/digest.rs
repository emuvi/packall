use crate::data::Body;

pub fn start(body: &Body) {
  println!("Digesting on: '{}'", body.home.root.display());
}
