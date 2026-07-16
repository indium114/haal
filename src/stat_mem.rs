use sysinfo::System;

pub fn total() -> f64 {
    let mut sys = System::new();
    sys.refresh_all();

    sys.total_memory() as f64 / 1_073_741_824.0
}

pub fn used() -> f64 {
    let mut sys = System::new();
    sys.refresh_all();

    sys.used_memory() as f64 / 1_073_741_824.0
}
