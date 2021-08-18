use crate::head;

impl head::Head<'_> {
	
	pub fn give(&self, path: &str) {
		println!("Giving to: {}", path);
	}
	
}
