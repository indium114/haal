use std::path::Path;

use sysinfo::Disks;

pub fn total() -> f64 {
    let disks = Disks::new_with_refreshed_list();

    let root = disks
        .list()
        .iter()
        .find(|d| d.mount_point() == Path::new("/"));

    if let Some(disk) = root {
        disk.total_space() as f64 / 1_073_741_824.0
    } else {
        0.0
    }
}

pub fn used() -> f64 {
    let disks = Disks::new_with_refreshed_list();

    let root = disks
        .list()
        .iter()
        .find(|d| d.mount_point() == Path::new("/"));

    if let Some(disk) = root {
        (disk.total_space() - disk.available_space()) as f64 / 1_073_741_824.0
    } else {
        0.0
    }
}
