mod commands;

use clap::{Parser, Subcommand};
use commands::list_containers;
use commands::pull_image;
use commands::run_container;
use commands::stop_container;
use commands::logs_container;
use commands::exec_in_container;

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
        #[arg(short='e', long="env", num_args=1..)]
        envs: Vec<String>,
        #[arg(long="volume", num_args=1..)]
        volumes: Vec<String>,
    },
    Stop {
        container_id: String,
    },
    Logs {
        container_id: String,
        #[arg(short, long)]
        follow: bool, // Follow logs in real time
    },
    Exec {
        container_id: String,
        #[arg(last = true)]
        cmd: Vec<String>,
    }
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
        Commands::Run { image , envs, volumes } => {
            run_container(&image, &envs, &volumes).await?;
        },
        Commands::Stop { container_id } => {
            stop_container(&container_id).await?;
        },
        Commands::Logs { container_id, follow } => {
            logs_container(&container_id, follow).await?;
        }
        Commands::Exec { container_id, cmd } => {
            exec_in_container(&container_id, &cmd).await?;
        }
    }
    
    Ok(())
}
