use sysinfo::System;

pub fn name() -> String {
    match System::name() {
        Some(name) => name,
        None => "unknown".to_string(),
    }
}

pub fn version() -> String {
    match System::os_version() {
        Some(version) => version,
        None => "unknown".to_string(),
    }
}

pub fn hostname() -> String {
    match System::host_name() {
        Some(host) => host,
        None => "unknown".to_string(),
    }
}
