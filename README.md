# Perky
**Key**-Value Kee**per**

---
Note: This package uses my [dictionaries package](https://github.com/Raforawesome/dictionaries-rs)
### What is Perky?
Perky is a simple key-value store that stores relational key-value pairs like a database using in-memory structs.  It can also optionally write these to file in a JSON-like format for easy, human-readable, permanent storage.

### What happens to file writes if you send too many requests?
In the future, Perky will use a queue system to handle scenarios like this.  However, currently Perky is single-threaded and writes are handled manually, so this won't be a problem yet.

### How do I use Perky?
To add it to your project, add this line to your dependencies in your `Cargo.toml`:
```toml
[dependencies]
perky = { git = "https://github.com/Raforawesome/perky" }
```
Perky behaves as a normal dictionary would.

As for usage, these are Perky's current functions:
#### Setting a key-value pair
```rust
use perky::Perky;

fn main() { 
    // Perky::new takes 2 parameters, a bool and the filename
    // the bool isn't used yet
    let perky = Perky::new(false, "perky_db");
    perky.set("test".to_string(), "value.to_string()");
}
```

#### Getting a value from a key
```rust
use perky::Perky;

fn main() { 
    // Perky::new takes 2 parameters, a bool and the filename
    // the bool isn't used yet
    let perky = Perky::new(false, "perky_db");
    perky.set("test".to_string(), "value.to_string()");
    
    // Perky.get returns a reference to a string, and takes a string slice as the argument
    let result: Option<&String> = perky.get("test");
}
```
#### Writing values to file
```rust
use perky::Perky;

fn main() { 
    // Perky::new takes 2 parameters, a bool and the filename
    // the bool isn't used yet
    let perky = Perky::new(false, "perky_db");
    perky.set("test".to_string(), "value".to_string());
    
    // Writes to the current directory
    // Uses file name provided when creating the Perky instance
    
    perky.write_file();
}
```
#### Reading values from file
```rust
use perky::Perky;

fn main() {
    // uses the same file name
    // creates a new perky instance
    let perky = Perky::from_file("perky_db");
}
```

### Bonus: Getting a key from a value
Since Perky makes its internal dictionary public, you can call functions on that directly, such as the `Dictionary.get_key()` function. 
#### Getting a value from a key
```rust
use perky::Perky;

fn main() { 
    // Perky::new takes 2 parameters, a bool and the filename
    // the bool isn't used yet
    let perky = Perky::new(false, "perky_db");
    perky.set("test".to_string(), "value.to_string()");
    
    // Dictionary.get_key returns a reference to a string, and takes a string slice as the argument
    let result: Option<&String> = perky.data.get_key("test");
}
```