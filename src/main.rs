mod cmd;

use clap::{Parser, Subcommand};

use cmd::check::check_cmd;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => {
            check_cmd();
        }
        None => {
            println!("On Default");
        }
    }
}
