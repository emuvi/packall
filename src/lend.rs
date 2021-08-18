use crate::head;

impl head::Head<'_> {
	
	pub fn lend(&self, path: &str) {
		println!("Lending to: {}", path);
	}
	
}
