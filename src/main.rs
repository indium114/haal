use mlua::Lua;

mod stat;
mod stat_wm;

fn main() {
    let mut lua = Lua::new();

    stat::stat(&mut lua);

    lua.load(
        r#"
            print("Window Manager: " .. wm.name)
        "#,
    )
    .exec()
    .unwrap();
}
