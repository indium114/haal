use std::fs::read_to_string;

pub fn name() -> String {
    read_to_string("/sys/devices/virtual/dmi/id/sys_vendor")
        .unwrap_or("unknown".to_string())
        .trim()
        .to_string()
}

pub fn model() -> String {
    read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .unwrap_or("unknown".to_string())
        .trim()
        .to_string()
}
