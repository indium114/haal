use colored_text::Colorize;
use mlua::{Lua, Table, Value};
use std::{fs, iter::repeat_n, process::exit};

mod stat;
mod stat_terminal;
mod stat_uptime;
mod stat_user;
mod stat_vendor;
mod stat_wm;

fn config_dir() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.config/haal"
}

fn load_logo() -> (Vec<String>, usize) {
    let contents = match fs::read_to_string(config_dir() + "/logo.txt") {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read logo at {}", config_dir() + "/logo.txt");
            println!("(make sure the file exists and is readable)");
            exit(1)
        }
    };

    let mut lines: Vec<String> = contents.lines().map(String::from).collect();
    let longest = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for line in &mut lines {
        let padding = longest - line.len();
        line.extend(repeat_n(' ', padding));
    }

    (lines, longest)
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
    let mut stats: Vec<String> = Vec::new();
    let (logo, spaces) = load_logo();
    let blank = " ".repeat(spaces);

    stat::stat(&mut lua);
    lua.load(config).exec().unwrap();

    let main_table: Table = match lua.globals().get("stats") {
        Ok(func) => func,
        Err(_) => {
            println!("No main function specified.");
            exit(1)
        }
    };

    let logo_colour: String = match lua.globals().get("logo_colour") {
        Ok(colour) => colour,
        Err(_) => {
            println!("No 'logo_colour' specified");
            println!("(this value should be a string hex code)");
            exit(1);
        }
    };

    for p in main_table.pairs::<Value, String>() {
        let (_, i) = p.unwrap();
        stats.push(i);
    }

    for i in 0..logo.len().max(stats.len()) {
        let logo_line = logo.get(i).map_or(blank.as_str(), |b| b.as_str());
        let stat_line = stats.get(i).map_or("", |s| s.as_str());

        println!("{} {}", logo_line.hex(&logo_colour), stat_line);
    }
}
