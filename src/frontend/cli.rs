use clap::{Parser};
use log::{info};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    path: String,
}

pub fn run_cli() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let cli = Cli::parse();

    info!("URL: {}", cli.url);
    info!("Path: {}", cli.path);
}
