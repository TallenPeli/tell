use clap::{Arg, command};
use mlua::{HookTriggers, Lua, VmState};
use std::path::PathBuf;
use std::{env, fs};

fn get_absolute_path(script: &str) -> Result<PathBuf, std::io::Error> {
    let path = fs::canonicalize(script)?;
    return Ok(path);
}

// TODO: Implement a function to get the distro, pkg_manager, and arch dynamically instead of hardcoding them.
fn get_distro() -> String {
    return "arch".to_string();
}

fn get_pkg_manager() -> String {
    return "pacman".to_string();
}

fn get_arch() -> String {
    return "x86_64".to_string();
}

fn main() {
    let matches = command!().arg(Arg::new("script")).get_matches();

    // Get the script argument from the command line
    let script = matches
        .get_one::<String>("script")
        .expect("Err: No script provided.\nUsage: tell <script>");

    // Get the absolute path of the script
    let abs_path = get_absolute_path(script)
        .map_err(|e| {
            eprintln!("'{}': {}", script, e);
            std::process::exit(1);
        })
        .unwrap();

    println!("Script path: {:?}", abs_path);

    // Read the contents of the script file
    let contents = fs::read_to_string(abs_path.clone())
        .map_err(|e| {
            eprintln!("'{}': {}", abs_path.clone().display(), e);
            std::process::exit(1);
        })
        .unwrap();

    let lua = Lua::new();

    let system = lua
        .create_table_from([
            ("distro", get_distro()),
            ("pkg_manager", get_pkg_manager()),
            ("arch", get_arch()),
        ])
        .unwrap();

    let ping = lua
        .create_function(|_, str: String| {
            if str == "ping" {
                println!("pong");
            } else {
                println!(":(");
            }
            Ok(())
        })
        .unwrap();

    let tell = lua.create_table().unwrap();
    tell.set("system", system).unwrap();
    tell.set("ping", ping).unwrap();

    // Allows you to add `require("tell")` in Lua scripts to access the `tell` module
    lua.register_module("tell", tell.clone()).unwrap();
    lua.globals().set("tell", tell).unwrap();

    lua.load(contents.as_str()).exec().unwrap();
}
