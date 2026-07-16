use std::env::var;

pub fn user() -> String {
    var("USER").unwrap_or_else(|_| "unknown".to_string())
}
