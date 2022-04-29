pub mod data_struct;
pub mod perky;

#[cfg(test)]
mod tests {
    use crate::data_struct::dictionary::Dictionary;
    #[test]
    fn basic_add() {
        let mut dict: Dictionary = Dictionary::new();

        dict.set(String::from("test"), String::from("value"));

        if let Some(s) = dict.get("test") {
            println!("{}", s);
        } else {
            println!("No match found.");
        };
    }
    #[test]
    fn remove_test() {
        let mut dict: Dictionary = Dictionary::new();

        dict.set(String::from("test"), String::from("value"));

        if let Some(s) = dict.get("test") {
            println!("{}", s);
        } else {
            println!("No match found.");
        };

        dict.remove("test");

        if let Some(s) = dict.get("test") {
            println!("{}", s);
        } else {
            println!("No match found.");
        };
    }
    #[test]
    fn multiple_keys() {
        let mut dict: Dictionary = Dictionary::new();
        dict.set("test".to_string(), "value".to_string());
        dict.set("test2".to_string(), "value2".to_string());
        dict.set("test3".to_string(), "value3".to_string());

        if let Some(s) = dict.get("test2") {
            println!("{}", s);
        } else {
            println!("No value found!");
        }
    }
    #[test]
    fn multiple_keys_remove() {
        let mut dict: Dictionary = Dictionary::new();
        dict.set("test".to_string(), "value".to_string());
        dict.set("test2".to_string(), "value2".to_string());
        dict.set("test3".to_string(), "value3".to_string());

        if let Some(s) = dict.get("test2") {
            println!("{}", s);
        } else {
            println!("No value found.");
        }

        dict.remove("test2");

        if let Some(s) = dict.get("test2") {
            println!("{}", s);
        } else {
            println!("No value found.");
        }
        if let Some(s) = dict.get("test3") {
            println!("{}", s);
        } else {
            println!("No value found.");
        }
    }
}
