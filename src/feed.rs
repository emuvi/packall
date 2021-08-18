use crate::head;

impl head::Head<'_> {
	
	pub fn feed(&self, path: &str) {
		println!("Feeding from: {}", path);
	}
	
}
