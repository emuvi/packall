use crate::body;

impl body::Head {
	
	pub fn lend(&self, path: &str) {
		println!("Lending to: '{}'", path);
	}
	
}
