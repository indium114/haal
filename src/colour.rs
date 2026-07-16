use mlua::{Function, Lua};

pub fn colours(state: &mut Lua) {
    // MARK: normal colours
    let black: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;30m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let red: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;31m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let green: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;32m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let yellow: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;33m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let blue: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;34m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let purple: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;35m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let cyan: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;36m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let white: Function = state
        .create_function(|_, a: String| Ok("\x1b[0;37m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();

    // MARK: bold colours
    let bold_black: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;30m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_red: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;31m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_green: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;32m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_yellow: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;33m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_blue: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;34m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_purple: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;35m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_cyan: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;36m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();
    let bold_white: Function = state
        .create_function(|_, a: String| Ok("\x1b[1;37m".to_string() + &a + "\x1b[0;37m"))
        .unwrap();

    let colour_table = state.create_table().unwrap();
    // add normal colours to table
    colour_table.set("black", black).unwrap();
    colour_table.set("red", red).unwrap();
    colour_table.set("green", green).unwrap();
    colour_table.set("yellow", yellow).unwrap();
    colour_table.set("blue", blue).unwrap();
    colour_table.set("purple", purple).unwrap();
    colour_table.set("cyan", cyan).unwrap();
    colour_table.set("white", white).unwrap();
    // add bold colours to table
    colour_table.set("boldBlack", bold_black).unwrap();
    colour_table.set("boldRed", bold_red).unwrap();
    colour_table.set("boldGreen", bold_green).unwrap();
    colour_table.set("boldYellow", bold_yellow).unwrap();
    colour_table.set("boldBlue", bold_blue).unwrap();
    colour_table.set("boldPurple", bold_purple).unwrap();
    colour_table.set("boldCyan", bold_cyan).unwrap();
    colour_table.set("boldWhite", bold_white).unwrap();
    state.globals().set("colour", colour_table).unwrap();
}
