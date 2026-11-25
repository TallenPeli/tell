use clap::{Arg, command};
use mlua::{HookTriggers, Lua, VmState};
use std::path::PathBuf;
use std::{env, fs};

fn get_absolute_path(script: &str) -> Result<PathBuf, std::io::Error> {
    let path = fs::canonicalize(script)?;
    return Ok(path);
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
    let greet = lua
        .create_function(|_, name: String| {
            println!("Hello, {}!", name);
            Ok(())
        })
        .unwrap();

    lua.globals().set("greet", greet).unwrap();

    lua.load(contents.as_str()).exec().unwrap();
}
