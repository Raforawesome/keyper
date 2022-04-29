pub struct Dictionary {
    keys: Vec<String>,
    values: Vec<String>
}

impl Dictionary {
    fn new() -> Dictionary {
        Dictionary {
            keys: Vec::new(),
            values: Vec::new()
        }
    }
    fn set(&mut self, k: String, v: String) {
        self.keys.push(k);
        self.values.push(v);
    }
    fn get(&self, k: &str) -> Option<&String> {
        let keys: &Vec<String> = &self.keys;
        let i = &keys.iter().position(|v| v == k);
        if let Some(n) = i {
            Some(&self.values[*n])
        } else {
            None
        }
    }
    fn remove(&self, k: &str) -> Result<_, _> {
        let keys: &Vec<String> = &self.keys;
        let values: &Vec<String> = &self.values;
        let i = keys.iter().position(|v| v == k)?;
    }
}