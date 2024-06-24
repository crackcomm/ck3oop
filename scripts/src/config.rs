use ::clap::{Arg, ArgAction, Command};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let root = Command::new("root")
        .subcommand_required(true)
        .arg_required_else_help(true);

    let env = Command::new("env")
        .about("Manage environment variables")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("get")
                .about("Get an environment variable")
                .arg(
                    Arg::new("key")
                        .help("The key to set")
                        .required(true)
                        .action(ArgAction::Set),
                ),
        );

    let matches = root.subcommand(env).get_matches();

    #[allow(clippy::single_match)]
    match matches.subcommand() {
        Some(("env", env)) => match env.subcommand() {
            Some(("get", get)) => {
                let key = get.get_one::<String>("key").context("get <key>")?;
                println!("{}", std::env::var(key)?);
                return Ok(());
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
