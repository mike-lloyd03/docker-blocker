use clap::{Parser, Subcommand};
use std::process::exit;
mod command;
use docker_blocker::{load_config, Config};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    // /// Suppress standard output
    // #[clap(short, long)]
    // quiet: bool,

    // /// Arguments to pass to restic
    // #[clap(short, long)]
    // args: Option<String>,
    /// Alternate configuration file to use
    #[clap(short, long, value_name = "FILE", default_value = "config.yaml")]
    config_file: String,

    #[clap(subcommand)]
    command: Command,
}

pub struct App {
    args: Args,
    config: Config,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a new repo
    Reload,
    /// Run a backup job now
    Disable,
    // /// Check the condition of all configured repos
    // Check {
    //     /// The repository to check
    //     repo: Option<String>,
    // },
}

fn main() {
    let args = Args::parse();
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Unable to load the configuration file. {}", e);
            exit(1);
        }
    };
    // let app = App { args, config };

    match args.command {
        Command::Reload => self::command::reload(&config),
        Command::Disable => todo!(),
    }
}
