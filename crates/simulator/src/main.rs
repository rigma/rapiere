use clap::Parser;
use cli::Cli;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod cli;
mod commands;
mod generation;
mod models;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("unable to attach tracing subscriber");

    let options = Cli::parse();
    let seed = options.seed();

    tracing::info!(seed = seed, "starting simulation");
    commands::run_command(options.command, seed);
    tracing::info!(seed = seed, "simulation complete");
}
