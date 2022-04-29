use crate::data_struct::dictionary::Dictionary;
use std::fs;
use std::path::PathBuf;

pub struct Keyper {
    auto_write: bool,
    file_name: String,
    data: Dictionary,
    mutex: bool,
    queue: Vec<u8>
}

impl Default for Keyper {
    fn default() -> Keyper {
        Keyper::new(false, "keyper_db".to_string())
    }
}

impl Keyper {
    pub fn new<T: ToString>(auto_write: bool, file_name: T) -> Keyper {
        let data: Dictionary = Dictionary::new();
        if auto_write {
            let res = fs::File::create(PathBuf::from(file_name.to_string()));
            if let Err(e) = res {
                println!("WARNING: Error in creating file for new Keyper instance :: {:?}", e);
            }
        }
        Keyper {
            auto_write,
            file_name: file_name.to_string(),
            data,
            mutex: false,
            queue: Vec::new()
        }
    }
}

impl Keyper {
    pub fn set(&mut self, k: String, v: String) {
        self.data.set(k, v);
        if self.auto_write {
            if self.mutex {
                self.queue.push(0);
            } else {
                self.write_values();
            }
        }
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.data.get(k)
    }
    pub fn remove(&mut self, k: &str) -> Option<()> {
        let res: Option<()> = self.data.remove(k);
        if self.auto_write {
            if self.mutex {
                self.queue.push(0);
            } else {
                self.write_values();
            }
        }
        res
    }
    fn write_values(&mut self) {
        self.mutex = true;
        self.mutex = false;
    }
}