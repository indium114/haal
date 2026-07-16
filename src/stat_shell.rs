use std::env::var;

pub fn shell() -> String {
    let path = var("SHELL").unwrap_or("unknown".to_string());
    match path.rfind("/") {
        Some(idx) => path[idx + 1..].to_string(),
        None => path,
    }
}
