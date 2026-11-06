use sysinfo::System;

pub fn name() -> String {
    match System::name() {
        Ok(name) => name,
        Err(_) => "unknown".to_string(),
    }
}

pub fn version() -> String {
    match System::os_version() {
        Ok(version) => version,
        Err(_) => "unknown".to_string(),
    }
}

pub fn hostname() -> String {
    match System::host_name() {
        Ok(host) => host,
        Err(_) => "unknown".to_string(),
    }
}

pub fn kernel() -> String {
    match System::kernel_version() {
        Ok(kernel) => kernel,
        Err(_) => "unknown".to_string(),
    }
}
