use std::fs::File;
use std::io::Write;
use std::str;
extern crate dirs;

use clap::Command;
use regex::Regex;

fn setup() {
    let buf = [0; 1000];
    let dir = std::env::current_dir().unwrap();
    println!("{}", dir.to_str().unwrap());
    let home = dirs::home_dir().unwrap();
    let mut prof_path = home.clone();
    prof_path.push(".profile");
    let mut phphp_path = home;
    phphp_path.push(".phphp");
    let mut r = File::create(prof_path).unwrap();
    let _ = std::fs::create_dir_all(&phphp_path).unwrap();
    let prof_content = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(_) => panic!("Invalid UTF8 profile"),
    };
    let new_prof = prof_content.to_string()
        + &format!("\nPATH=\"{}:$PATH\"\n", phphp_path.as_path().display());
    File::write(&mut r, new_prof.as_bytes()).expect("setup failed");
    println!("{}", prof_content);
}

fn main() {
    let matches = Command::new("phphp")
        .about("php version manager")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Kodai Kabasawa kbwo")
        .subcommand(Command::new("setup"))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("use"))
        .subcommand(Command::new("clean"))
        .subcommand(Command::new("install"))
        .subcommand(Command::new("recompile"))
        .subcommand(Command::new("uninstall"))
        .subcommand(Command::new("install-dependencies"))
        .get_matches();
    match matches.subcommand() {
        Some(("setup", _)) => {
            setup();
        }
        _ => {
            unimplemented!();
        }
    }
}
