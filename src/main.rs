use std::fs::File;
use std::io::{Error, Write};
use std::str;
extern crate dirs;

use clap::{Arg, Command};
use regex::Regex;

fn setup() {
    let buf = [0; 1000];
    let home = dirs::home_dir().unwrap();
    let mut prof_path = home.clone();
    prof_path.push(".profile");
    let mut phen_path = home;
    phen_path.push(".phen");
    let mut r = File::create(prof_path).unwrap();
    let _ = std::fs::create_dir_all(&phen_path).unwrap();
    let prof_content = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(_) => panic!("Invalid UTF8 profile"),
    };
    let new_prof =
        prof_content.to_string() + &format!("\nPATH=\"{}:$PATH\"\n", phen_path.as_path().display());
    File::write(&mut r, new_prof.as_bytes()).expect("setup failed");
    println!("{}", prof_content);
}

async fn install(version: &str) {
    let reg = Regex::new(r"(^\d+\.\d+\.\d+$)").unwrap();
    if !reg.is_match(version) {
        panic!("Unexpected version format provided: {}", version);
    }
    let home = dirs::home_dir().unwrap();
    let phen_path = home.join(".phen");
    // if Path::new(phen_path.as_path()).is_dir() {
    //     panic!(
    //         "It looks like {} may already be installed, attempt clean up?",
    //         version
    //     );
    // }
    println!("Downloading {}", version);
    let url = format!("https://www.php.net/distributions/php-{}.tar.gz", version);
    let response = reqwest::get(url)
        .await
        .unwrap_or_else(|_| panic!("Could not find {} via php.net releases", version));
    let content = response.text().await.unwrap();
    let tmp_dir = phen_path.join("tmp");
    let install_dir = phen_path.join("lib");
    let _ = std::fs::create_dir(install_dir.as_path());
    let tmp_path = tmp_dir.join(format!("php-{}.tar.gz", version));
    let _ = std::fs::create_dir_all(tmp_dir);
    // let _ = copy(
    //     &mut content.as_bytes(),
    //     &mut File::create(tmp_path).unwrap(),
    // );
    println!("tmp_path, {}", &tmp_path.to_str().unwrap());
    let _ = std::fs::File::create(&tmp_path)
        .unwrap()
        .write_all(content.as_bytes());
    let _ = std::process::Command::new("tar")
        .args([
            "-xzf",
            tmp_path.to_str().unwrap(),
            "--strip-components=1 -C",
            install_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to handle tar");
    let _ = std::fs::remove_file(&tmp_path).expect("failed to remove tmp file");
    todo!("compile");
}

async fn compile(version: &str) {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = Command::new("phen")
        .about("php version manager")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Kodai Kabasawa kbwo")
        .subcommand(Command::new("setup"))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("use"))
        .subcommand(Command::new("clean"))
        .subcommand(
            Command::new("install")
                .arg_required_else_help(true)
                .arg(Arg::new("version").takes_value(true)),
        )
        .subcommand(Command::new("recompile"))
        .subcommand(Command::new("uninstall"))
        .subcommand(Command::new("install-dependencies"))
        .get_matches();
    match matches.subcommand() {
        Some(("setup", _)) => {
            setup();
        }
        Some(("install", arg)) => {
            let version = arg.value_of("version").unwrap();
            install(version).await;
        }
        _ => {
            unimplemented!();
        }
    }
    Ok(())
}
