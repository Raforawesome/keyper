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
	file: Option<fs::File>,
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
		let mut t = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
		t.push(PathBuf::from(file_name.to_string()));
		let data: Dictionary = Dictionary::new();
		let res = fs::File::create(t);
		let mut writable: Option<fs::File> = None;
		if let Err(e) = res {
			println!("WARNING: Error in creating file for new Keyper instance :: {:?}", e);
		} else if let Ok(f) = res {
			writable = Some(f);
		}
		Perky {
			auto_write,
			file_name: file_name.to_string(),
			data,
			mutex: false,
			file: writable
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
	fn write_values(&mut self) {
		if !self.mutex {
			if let Some(f) = &self.file {
				self.mutex = true;
				let file_name = &self.file_name;
				let mut t = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
				t.push(PathBuf::from(file_name.to_string()));
				let mut file: String = String::new();
				for i in 0..self.data.keys.len() {
					let k = &self.data.keys[i];
					let v = &self.data.values[i];
					file = file + k.as_str() + "|PERKY_SEP|" + v.as_str() + "\n";
				}
				let res = fs::write(t, file);
				if res.is_err() {
					println!("Perky :: WARNING: Writing to file failed");
				}
				self.mutex = false;
			}
		}
	}
}