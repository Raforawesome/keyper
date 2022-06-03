#![allow(dead_code)]
#![allow(clippy::redundant_pattern_matching)]
use dictionaries::{Dictionary, RemoveError};
use std::fs;
use std::path::PathBuf;
// use std::thread::{self, JoinHandle};
// use std::time::Duration;

#[derive(Debug)]
pub struct Perky {
	auto_write: bool,
	file_name: String,
	pub data: Dictionary,
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
	pub fn from_file(file_name: &str) -> Perky {
		let owned = String::from(file_name);
		let mut path: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
		path.push(&owned);
		let file_obj = fs::File::open(&path).unwrap();
		let file = fs::read_to_string(&path);

		let mut data: Dictionary = Dictionary::new();
		if let Ok(s) = file {
			let lines = s.lines();
			for line in lines {
				let split = line.split("|PERKY_SEP|").collect::<Vec<&str>>();
				data.set(String::from(split[0]), String::from(split[1]));
			}
		} else {
			println!("Perky :: WARNING: Error in reading file.")
		}

		Perky {
			auto_write: false,
			file_name: owned,
			data,
			mutex: false,
			file: Some(file_obj)
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
	pub fn remove(&mut self, k: &str) -> Result<(), RemoveError> {
		let res: Result<(), RemoveError> = self.data.remove(k);
		// if self.auto_write {
		// 	if self.mutex {
		// 		self.queue.push(0);
		// 	} else {
		// 		self.write_values();
		// 	}
		// }
		res
	}
	pub fn write_file(&mut self) {
		if !self.mutex {
			if let Some(_) = &self.file {
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
				let res = fs::write(&t, file);
				if res.is_err() {
					println!("Perky :: WARNING: Writing to file failed");
				}
				// println!("Written to {:?}", &t);
				self.mutex = false;
			}
		}
	}
}