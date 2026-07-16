use std::env::var;

pub fn wm() -> String {
    var("XDG_CURRENT_DESKTOP")
        .unwrap_or_else(|_| var("DESKTOP_SESSION").unwrap_or_else(|_| "unknown".to_string()))
}
