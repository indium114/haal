use sysinfo::System;

pub fn name() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();

    sys.cpus()[0].brand().to_string()
}

pub fn freq() -> f64 {
    let mut sys = System::new_all();

    sys.refresh_all();

    sys.cpus()[0].frequency() as f64 / 1000.0
}
