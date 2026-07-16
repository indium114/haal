use humantime::format_duration;
use std::time::Duration;
use system_uptime::get_os_uptime_duration;

pub fn uptime() -> String {
    let uptime = Duration::from_secs(get_os_uptime_duration().unwrap().as_secs());
    format_duration(uptime).to_string()
}
