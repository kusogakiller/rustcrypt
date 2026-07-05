mod core;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate { size: u32 },
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { size } => {
            println!("{}", core::generate(size));
        }

        Commands::Version => {
            println!("rustcrypt 0.2.0");
        }
    }
}
