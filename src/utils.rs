use std::time::{SystemTime, UNIX_EPOCH};

// Returns the current UNIX_EPOCH timestamp in seconds (u64)
// If the system clock fails, returns an error (String)
pub fn current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_secs(),
        Err(e) => {
            eprintln!("This went backwards... ERROR: {:?}", e);
            0 // Fallback value
        }
    }
}
