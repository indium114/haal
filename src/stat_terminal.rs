use std::env::var;

pub fn terminal() -> String {
    var("TERM").unwrap_or("unknown".to_string())
}
