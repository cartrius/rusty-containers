mod commands;

use clap::{Parser, Subcommand};
use commands::list_containers;
use commands::pull_image;
use commands::run_container;
use commands::stop_container;

#[derive(Parser)]
#[command(name = "rusty-containers")]
#[command(about = "A simple Rust CLI to manage containers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ls,
    Pull {
        image: String,
    },
    Run {
        image: String,
    },
    Stop {
        container_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => {
            list_containers().await?;
        },
        Commands::Pull { image } => {
            pull_image(&image).await?;
        },
        Commands::Run { image } => {
            run_container(&image).await?;
        },
        Commands::Stop { container_id } => {
            stop_container(&container_id).await?;
        }
    }
    
    Ok(())
}
