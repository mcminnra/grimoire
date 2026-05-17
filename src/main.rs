mod cmd;

use clap::{Parser, Subcommand};

use cmd::{check::check_cmd, list::list_cmd};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Check,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            let games_directory = shellexpand::tilde("~/org/notes/games");
            if let Err(e) = list_cmd(&games_directory) {
                eprintln!("{e:#?}");
                println!("Unable to list games!");
            }
        }
        Some(Commands::Check) => {
            check_cmd();
        }
        None => {}
    }
}
