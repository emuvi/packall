use crate::head;

impl head::Head<'_> {
	
	pub fn digest(&self) {
		println!("Digesting on: {}", self.body);
	}
	
}
