use clap::{Parser, Subcommand};

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
            println!("Listing containers (ph)");
        },
        Commands::Pull { image } => {
            println!("Pulling image: {image} (ph)");
        },
        Commands::Run { image } => {
            println!("Running container from image: {image} (ph)");
        },
        Commands::Stop { container_id } => {
            println!("Stopping container with ID: {container_id} (ph)");
        }
    }
    
    Ok(())
}
