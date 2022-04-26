use clap::Command;

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
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }
}
