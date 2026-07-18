use colored_text::Colorize;
use mlua::{Lua, Table, Value};
use std::{fs, iter::repeat_n, process::exit};

mod stat;
mod stat_cpu;
mod stat_disk;
mod stat_gpu;
mod stat_mem;
mod stat_os;
mod stat_shell;
mod stat_terminal;
mod stat_uptime;
mod stat_user;
mod stat_vendor;
mod stat_wm;

mod colour;

fn config_dir() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home + "/.config/haal"
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            while let Some(&x) = chars.peek() {
                chars.next();
                if x == 'm' {
                    break;
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn visible_width(s: &str) -> usize {
    strip_ansi(s).chars().count()
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
    let longest = lines.iter().map(|l| visible_width(l)).max().unwrap_or(0);

    for line in &mut lines {
        let padding = longest - visible_width(line);
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
    colour::colours(&mut lua);
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
