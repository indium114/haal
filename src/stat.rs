use mlua::Lua;

use crate::stat_user;
use crate::stat_wm;

pub fn stat(state: &mut Lua) {
    let user_table = state.create_table().unwrap();
    let wm_table = state.create_table().unwrap();

    wm_table.set("name", stat_wm::wm()).unwrap();
    user_table.set("name", stat_user::user()).unwrap();

    state.globals().set("wm", wm_table).unwrap();
    state.globals().set("user", user_table).unwrap();
}
