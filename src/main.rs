use std::{fs, process::exit};

use mlua::{Function, Lua};

mod stat;
mod stat_wm;

fn config_dir() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.config/haal"
}

fn main() {
    let mut lua = Lua::new();
    let config = match fs::read_to_string(config_dir() + "/init.lua") {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("Failed to read config at {}", config_dir() + "/init.lua");
            println!("(make sure the file exists and is readable)");
            exit(1);
        }
    };

    stat::stat(&mut lua);
    lua.load(config).exec().unwrap();

    let main_func: Function = match lua.globals().get("main") {
        Ok(func) => func,
        Err(_) => {
            println!("No main function specified.");
            exit(1)
        }
    };

    main_func.call::<()>(()).unwrap();
}
