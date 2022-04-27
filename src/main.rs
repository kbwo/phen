use std::fs::File;
use std::io::{Error, Read, Write};
use std::str;
extern crate dirs;

use clap::{Arg, Command};
use log::info;
use regex::Regex;

struct Conf {
    version: String,
    phen_path: std::path::PathBuf,
    prof_path: std::path::PathBuf,
    install_dir: std::path::PathBuf,
    tmp_dir: std::path::PathBuf,
}

impl Conf {
    fn new(version: &str) -> Self {
        let home = dirs::home_dir().unwrap();
        let prof_path = home.clone();
        let phen_path = home.join(".phen");
        let install_dir = phen_path.join("lib");
        let tmp_dir = phen_path.join("tmp");
        Self {
            version: version.to_string(),
            phen_path,
            prof_path,
            install_dir,
            tmp_dir,
        }
    }
}

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

async fn install(conf: &Conf) {
    validate_version(&conf.version);
    println!("Downloading {}", conf.version);
    let url = format!(
        "https://www.php.net/distributions/php-{}.tar.gz",
        conf.version
    );
    let response = reqwest::get(url)
        .await
        .unwrap_or_else(|_| panic!("Could not find {} via php.net releases", conf.version));
    let content = response.text().await.unwrap();
    let _ = std::fs::create_dir(conf.install_dir.as_path());
    let tmp_path = conf.tmp_dir.join(format!("php-{}.tar.gz", conf.version));
    let _ = std::fs::create_dir_all(conf.tmp_dir.clone());
    println!("tmp_path, {}", &tmp_path.to_str().unwrap());
    let _ = std::fs::File::create(&tmp_path)
        .unwrap()
        .write_all(content.as_bytes());
    let _ = std::process::Command::new("tar")
        .args([
            "-xzf",
            tmp_path.to_str().unwrap(),
            "--strip-components=1 -C",
            conf.install_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to handle tar");
    let _ = std::fs::remove_file(&tmp_path).expect("failed to remove tmp file");
    todo!("compile");
}

fn validate_version(version: &str) {
    let reg = Regex::new(r"(^\d+\.\d+\.\d+$)").unwrap();
    if !reg.is_match(version) {
        panic!("Unexpected version format provided: {}", version);
    }
}

async fn compile(conf: &Conf) {
    validate_version(&conf.version);
    info!("Compiling");
    let install_dir_path = conf.install_dir.as_path().to_str().unwrap();
    let _ = std::process::Command::new("./configure")
        .current_dir(&conf.install_dir)
        .args([
            format!("--prefix={}", install_dir_path).as_str(),
            "--with-openssl",
            "--with-pcre-jit",
            "--with-zlib",
            "--enable-bcmath",
            "--with-bz2",
            "--enable-calendar",
            "--with-curl",
            "--with-gd",
            "--enable-mbstring",
            "--with-mysqli",
            "--with-pdo-mysql",
            "--with-pdo-pgsql",
            "--with-xsl",
            "--enable-zip",
            "--with-zip",
            "--without-pear",
        ])
        .output()
        .expect("failed to configure");
    let _ = std::process::Command::new("make -j 16 && make install")
        .output()
        .expect("failed to compile");
    todo!("add version")
}

fn add_version(conf: &Conf) {
    let mut buf = String::new();
    // let bin_path = conf.install_dir.as_path().join("bin");
    let mut ini_file = File::options()
        .read(true)
        .write(true)
        .open(conf.phen_path.as_path().join("etc"))
        .unwrap();
    let version_info = format!("versions[\"{}\"]", &conf.version);
    let reg = Regex::new(&version_info).unwrap();
    let _ = ini_file.read_to_string(&mut buf).unwrap();
    if reg.is_match(&buf) {
        unimplemented!()
        // reg.replace(format!(r"versions[\"{}\"]\".+\"", conf.version).as_str(), format!("versions[\"{}\"]=\"{}\"", conf.version, bin_path.to_str()).as_str());
    }
}

fn write_config() {}

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
            let conf = Conf::new(version);
            install(&conf).await;
        }
        _ => {
            unimplemented!();
        }
    }
    Ok(())
}
