use anyhow::Result;
use clap::Parser;

/// BranchFlow CLI
#[derive(Parser, Debug)]
#[command(name = "branchflow")]
#[command(about = "A modern Git workflow CLI", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("BranchFlow CLI starting in verbose mode");
    } else {
        println!("BranchFlow CLI");
    }

    Ok(())
}
