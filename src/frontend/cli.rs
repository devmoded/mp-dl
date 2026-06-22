use clap::{Parser};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    path: String,
}

pub fn run_cli() {
    let cli = Cli::parse();

    println!("URL: {}", cli.url);
    println!("Path: {}", cli.path);
}
