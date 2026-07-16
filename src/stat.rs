use mlua::Lua;

use crate::stat_uptime;
use crate::stat_user;
use crate::stat_vendor;
use crate::stat_wm;

pub fn stat(state: &mut Lua) {
    let user_table = state.create_table().unwrap();
    let wm_table = state.create_table().unwrap();
    let vendor_table = state.create_table().unwrap();
    let uptime_table = state.create_table().unwrap();

    wm_table.set("name", stat_wm::wm()).unwrap();
    user_table.set("name", stat_user::user()).unwrap();
    vendor_table.set("name", stat_vendor::name()).unwrap();
    vendor_table.set("model", stat_vendor::model()).unwrap();
    uptime_table.set("uptime", stat_uptime::uptime()).unwrap();

    state.globals().set("wm", wm_table).unwrap();
    state.globals().set("user", user_table).unwrap();
    state.globals().set("vendor", vendor_table).unwrap();
    state.globals().set("uptime", uptime_table).unwrap();
}
