pub struct Dictionary {
    keys: Vec<String>,
    values: Vec<String>
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            keys: Vec::new(),
            values: Vec::new()
        }
    }
    pub fn set(&mut self, k: String, v: String) {
        self.keys.push(k);
        self.values.push(v);
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        let keys: &Vec<String> = &self.keys;
        let i = &keys.iter().position(|v| v == k);
        if let Some(n) = i {
            Some(&self.values[*n])
        } else {
            None
        }
    }
    pub fn remove(&mut self, k: &str) -> Option<_> {
        let keys: &mut Vec<String> = &mut self.keys;
        let values: &mut Vec<String> = &mut self.values;
        let i = keys.iter().position(|v| v == k)?;
        keys.remove(i);
        values.remove(i);
        Some(())
    }
}