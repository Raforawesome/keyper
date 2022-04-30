mod perky;
pub use perky::Perky;

#[cfg(test)]
mod tests {
	use crate::perky::Perky;
	#[test]
	fn write_test() {
		let mut perky = Perky::new(false, "perky_db".to_string());
		perky.set("test1".to_string(), "value1".to_string());
		perky.set("test2".to_string(), "value2".to_string());
		perky.set("test3".to_string(), "value3".to_string());
		perky.write_file();
	}
	#[test]
	fn open_write_test() {
		let mut perky = Perky::from_file("perky_db");
		for (k, v) in perky.data.iter() {
			println!("{}, {}", k, v);
		}
		perky.set("key4".to_string(), "value4".to_string());
		perky.write_file();
	}
}
