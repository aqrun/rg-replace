use cli::{Cli};
use clap::{Parser};
use cmd::{
    default_run,
};

mod cli;
mod cmd;
mod models;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    default_run(&cli);
}