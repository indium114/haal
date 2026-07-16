use mlua::Lua;

use crate::stat_wm;

pub fn stat(state: &mut Lua) {
    let wm_table = state.create_table().unwrap();

    wm_table.set("name", stat_wm::wm()).unwrap();

    state.globals().set("wm", wm_table).unwrap();
}
