
pub struct Head<'a> {
  pub body: &'a str,
  pub speed: usize,
  pub clean: bool,
}

impl Head<'_> {

	pub fn start(&self) {
		println!("Packall starting...");
		println!("Body: {}", self.body);
		println!("Speed: {}", self.speed);
		println!("Clean: {}", self.clean);
	}

}

