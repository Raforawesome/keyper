use dictionaries::Dictionary;
use std::fs;
use std::path::PathBuf;
// use std::thread::{self, JoinHandle};
// use std::time::Duration;

pub struct Perky {
	auto_write: bool,
	file_name: String,
	data: Dictionary,
	mutex: bool,
	// queue: Vec<u8>,
	// queue_handle: Option<JoinHandle<()>>
}

impl Default for Perky {
	fn default() -> Perky {
		Perky::new(false, "perky_db".to_string())
	}
}

// Associated functions
impl Perky {
	pub fn new<T: ToString>(auto_write: bool, file_name: T) -> Perky {
		let data: Dictionary = Dictionary::new();
		if auto_write {
			let res = fs::File::create(PathBuf::from(file_name.to_string()));
			if let Err(e) = res {
				println!("WARNING: Error in creating file for new Keyper instance :: {:?}", e);
			}
		}
		Perky {
			auto_write,
			file_name: file_name.to_string(),
			data,
			mutex: false,
			// queue: Vec::new(),
			// queue_handle: None
		}
	}
}

// Object-specific methods
impl Perky {
	pub fn set(&mut self, k: String, v: String) {
		self.data.set(k, v);
		// if self.auto_write {
		// 	if self.mutex {
		// 		self.queue.push(0);
		// 	} else {
		// 		self.write_values();
		// 	}
		// }
	}
	pub fn get(&self, k: &str) -> Option<&String> {
		self.data.get(k)
	}
	pub fn remove(&mut self, k: &str) -> Option<()> {
		let res: Option<()> = self.data.remove(k);
		// if self.auto_write {
		// 	if self.mutex {
		// 		self.queue.push(0);
		// 	} else {
		// 		self.write_values();
		// 	}
		// }
		res
	}
	// fn write_values(&mut self) {
	// 	self.mutex = true;
	// 	self.mutex = false;
	// }
}