use mlua::Lua;

use crate::stat_disk;
use crate::stat_gpu;
use crate::stat_mem;
use crate::stat_os;
use crate::stat_shell;
use crate::stat_terminal;
use crate::stat_uptime;
use crate::stat_user;
use crate::stat_vendor;
use crate::stat_wm;

pub fn stat(state: &mut Lua) {
    let user_table = state.create_table().unwrap();
    let wm_table = state.create_table().unwrap();
    let vendor_table = state.create_table().unwrap();
    let uptime_table = state.create_table().unwrap();
    let terminal_table = state.create_table().unwrap();
    let shell_table = state.create_table().unwrap();
    let os_table = state.create_table().unwrap();
    let mem_table = state.create_table().unwrap();
    let gpu_table = state.create_table().unwrap();
    let disk_table = state.create_table().unwrap();

    wm_table.set("name", stat_wm::wm()).unwrap();
    user_table.set("name", stat_user::user()).unwrap();
    vendor_table.set("name", stat_vendor::name()).unwrap();
    vendor_table.set("model", stat_vendor::model()).unwrap();
    uptime_table.set("uptime", stat_uptime::uptime()).unwrap();
    terminal_table
        .set("name", stat_terminal::terminal())
        .unwrap();
    shell_table.set("name", stat_shell::shell()).unwrap();
    os_table.set("name", stat_os::name()).unwrap();
    os_table.set("version", stat_os::version()).unwrap();
    os_table.set("hostname", stat_os::hostname()).unwrap();
    os_table.set("kernel", stat_os::kernel()).unwrap();
    mem_table.set("total", stat_mem::total()).unwrap();
    mem_table.set("used", stat_mem::used()).unwrap();
    gpu_table.set("name", stat_gpu::name()).unwrap();
    disk_table.set("total", stat_disk::total()).unwrap();
    disk_table.set("used", stat_disk::used()).unwrap();

    state.globals().set("wm", wm_table).unwrap();
    state.globals().set("user", user_table).unwrap();
    state.globals().set("vendor", vendor_table).unwrap();
    state.globals().set("uptime", uptime_table).unwrap();
    state.globals().set("terminal", terminal_table).unwrap();
    state.globals().set("shell", shell_table).unwrap();
    state.globals().set("os", os_table).unwrap();
    state.globals().set("mem", mem_table).unwrap();
    state.globals().set("gpu", gpu_table).unwrap();
    state.globals().set("disk", disk_table).unwrap();
}
