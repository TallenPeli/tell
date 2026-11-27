use clap::{arg, command, value_parser};
use mlua::Lua;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const LUA_TYPES: &str = include_str!("../default/lua_types.lua");
const DEFAULT_CONFIG: &str = include_str!("../default/init.lua");

fn get_absolute_path(script: &str) -> Result<PathBuf, std::io::Error> {
    let path = fs::canonicalize(script)?;
    return Ok(path);
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("tell"))
}

fn get_config_file() -> Option<PathBuf> {
    let config_dir = get_config_dir().unwrap();
    Some(config_dir.join("init.lua"))
}

// Returns the values of `ID` from /etc/os-release
fn get_distro_id() -> Result<String, Error> {
    let mut file = File::open("/etc/os-release")?;
    let mut buff = String::new();

    file.read_to_string(&mut buff)?;

    // Parse out ID
    let mut id = String::new();
    for line in buff.lines() {
        if line.starts_with("ID=") {
            id = line.replace("ID=", "");
        }
    }

    // If NAME was not found, return unknown
    if id == String::new() {
        return Ok("unknown".to_string());
    }

    return Ok(id);
}

// TODO
fn command_exists(cmd: &str) -> bool {
    return false;
}

// TODO Just use a map for now...
fn get_pkg_manager() -> Option<&'static str> {
    // Known supported package managers...
    const MANAGERS: &[&str] = &["pacman", "apt", "dnf"];
    MANAGERS.iter().copied().find(|&pm| command_exists(pm))
}

// Get system archetecture
fn get_arch() -> String {
    if let Ok(output) = Command::new("uname").args(&["-m"]).output() {
        let arch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return arch;
    }
    return String::from("unknown");
}

fn config_init() -> Result<(), std::io::Error> {
    let config_dir = get_config_dir().unwrap();
    fs::create_dir_all(&config_dir)?;

    let init_lua_path = config_dir.join("init.lua");
    let lua_types_path = config_dir.join("types.lua");

    // Create init.lua
    if !init_lua_path.exists() {
        fs::write(init_lua_path.clone(), DEFAULT_CONFIG)?;
        println!("Creating {:?}", init_lua_path.clone()); // TODO debug function
    } else {
        println!("{:?} exists, skipping creation", init_lua_path.clone()) // TODO debug function
    }
    println!("{:?}", init_lua_path);

    // Create types.lua
    if !lua_types_path.exists() {
        fs::write(lua_types_path.clone(), LUA_TYPES)?;
        println!("Creating {:?}", lua_types_path.clone()); // TODO debug function
    }

    Ok(())
}

fn open_editor() -> Result<(), std::io::Error> {
    // Initialize the configuration if it hasn't been already
    config_init().expect("Unable to initialize default configuration");

    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        println!("$EDITOR not set. Using vi as a fallback.");
        "vi".to_string()
    });

    Command::new(editor)
        .arg(get_config_dir().unwrap().join("init.lua"))
        .status()
        .expect("Failed to open editor");

    Ok(())
}

fn main() {
    let app_m = command!()
        .arg(
            arg!(
                -d --debug "Turn debugging information on"
                )
                .required(false)
            )
        .subcommand(
            command!("apply")
                .about("Apply a Lua script to the system")
                .arg(arg!(<script> "The Lua script to apply")
                    .required(false)
                    .value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            command!("plan")
                .about("Prints what would happen without applying the Lua script")
                .arg(arg!(<script> "The Lua script to plan")
                    .required(false)
                    .value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            command!("sync")
                .about("Pulls updates (git pull) on all cached tell.cfg repositories and re-applies any changes")
                .arg(arg!(<script> "The Lua script to apply")
                    .required(false)
                    .value_parser(value_parser!(PathBuf))),
            )
        .subcommand(
            command!("edit")
                .about("Opens $XDG_CONFIG/init.lua with $EDITOR"),
            )
        .get_matches();

    // Initialize config at startup if it hasn't been already
    config_init().unwrap();

    let config_dir = get_config_dir()
        .expect("No configuration directory found. Manually pass the path to init.lua");

    let arch = get_arch();
    let script = get_config_file().expect("No init.lua found. Manually pass the path to init.lua");

    println!("Using config: {:?}", script); // TODO debug function

    match app_m.subcommand() {
        Some(("apply", sub_m)) => {
            println!("Used apply. sub_m: {:?}", sub_m);
        }
        Some(("plan", sub_m)) => {
            println!("Used plan")
        }
        Some(("sync", sub_m)) => {
            println!("Used sync")
        }
        Some(("edit", _)) => open_editor().unwrap_or_else(|e| {
            eprintln!("Failed to open editor: {}", e);
            std::process::exit(1);
        }),
        _ => {}
    }

    println!("{:?}", &script);
    let abs_path = script.clone();

    // Get the absolute path of the script
    // let abs_path = get_absolute_path(script)
    //     .map_err(|e| {
    //         eprintln!("'{}': {}", script, e);
    //         std::process::exit(1);
    //     })
    //     .unwrap();

    // Read the contents of the script file
    let contents = fs::read_to_string(abs_path.clone())
        .map_err(|e| {
            eprintln!("'{}': {}", abs_path.clone().display(), e);
            std::process::exit(1);
        })
        .unwrap();

    let lua = Lua::new();
    let distro_id = get_distro_id().unwrap();
    let pkg_manager = match get_pkg_manager() {
        Some(pm) => pm,
        None => "unknown",
    };

    let arch = get_arch();

    let system = lua
        .create_table_from([
            ("distro_id", distro_id),
            ("pkg_manager", pkg_manager.to_string()),
            ("arch", arch),
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
