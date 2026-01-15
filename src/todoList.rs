#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
    created_at: u64, // Unix timestamp (seconds)
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
