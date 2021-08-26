use crate::body;

impl body::Head {
	
	pub fn digest(&self) {
		println!("Digesting on: '{}'", self.body.display());
	}
	
}
