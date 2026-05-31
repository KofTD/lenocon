use clap::Parser;
use clap::Subcommand;

use lenocon_core::{CONSERVATION_FILE_PATH, read_status, set_status, toggle_status};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// set conservation mode off
    Disable,
    /// set conservation mode on
    Enable,
    /// check conservation mode status
    Status,
    /// toggle conservation mode
    Toggle,
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Enable => set_status(true).map(|_| true),
        Commands::Disable => set_status(false).map(|_| false),
        Commands::Status => read_status(),
        Commands::Toggle => toggle_status(),
    };

    match result {
        Ok(enabled) => println!("Conservation mode: {}", if enabled { "ON" } else { "OFF" }),
        Err(err) => eprintln!("Error: {} to file {}", err, CONSERVATION_FILE_PATH),
    }
}
