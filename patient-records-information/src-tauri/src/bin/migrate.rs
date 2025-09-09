use clap::{Parser, Subcommand};
use patient_records_information_lib::migrations::cli;

#[derive(Parser)]
#[command(name = "migrate")]
#[command(about = "A CLI tool for managing database migrations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all pending migrations
    Up,
    /// Rollback the last migration
    Down,
    /// Reset all migrations
    Reset,
    /// Show migration status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Up => {
            cli::run_migration_cli().await?;
        }
        Commands::Down => {
            cli::rollback_migration_cli().await?;
        }
        Commands::Reset => {
            cli::reset_migration_cli().await?;
        }
        Commands::Status => {
            cli::status_migration_cli().await?;
        }
    }

    Ok(())
}
