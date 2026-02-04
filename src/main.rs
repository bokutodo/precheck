use clap::{Parser, Subcommand};
use anyhow::Result;
mod git;

#[derive(Parser)]
#[command(name = "precheck")]
#[command(about = "A pre-commit code reviewer and commit message generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run code review checks on staged changes
    Review,
    /// Generate a commit message from staged changes
    Message,
    /// Run review, then commit with generated message
    Commit,
}

fn run_review() -> anyhow::Result<()> {
    use colored::Colorize;
    
    println!("{}", "Running code review checks...".blue().bold());
    
    let diff = git::get_staged_diff()?;
    let files = git::get_staged_files()?;
    
    println!("\n{} staged files:", "Found".green());
    for file in &files {
        println!("  - {}", file);
    }
    
    println!("\n{}", "Diff preview:".yellow());
    println!("{}", diff);
    
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Review => {
            match run_review() {
                Ok(()) => println!("✓ All checks passed!"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Message => {
            println!("Generating commit message...");
            // We'll implement this next
        }
        Commands::Commit => {
            println!("Running review and committing...");
            // We'll implement this next
        }
    }
}

