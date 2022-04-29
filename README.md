# Keyper
**Key**-Value **Keeper**

---
### What is Keyper?
Keyper is a simple key-value store that stores relational key-value pairs like a database using in-memory structs.  It can also optionally write these to file in a JSON-like format for easy, human-readable, permanent storage.

### What happens to file writes if you send too many requests?
Keyper uses a queue system to keep track of writes to execute in case it's currently busy writing to file.