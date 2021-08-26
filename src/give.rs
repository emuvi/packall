use crate::body;

impl body::Head {
	
	pub fn give(&self, path: &str) {
		println!("Giving to: '{}'", path);
	}
	
}
