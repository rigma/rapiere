use clap::Parser;
use cli::Cli;
use std::{backtrace::Backtrace, process::ExitCode};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod commands;
mod generation;
mod models;

fn main() -> ExitCode {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .with(fmt::layer())
        .init();

    let options = Cli::parse();
    let seed = options.seed();

    tracing::info!(seed = seed, "starting simulation");

    std::panic::set_hook(Box::new(move |info| {
        let payload = info.payload();
        if let Some(message) = payload.downcast_ref::<String>() {
            tracing::error!(message = message, "panic caught during simulation");
        } else if let Some(message) = payload.downcast_ref::<&str>() {
            tracing::error!(message = message, "panic caught during simulation");
        } else {
            tracing::error!("panic caught during simulation");
        }

        let backtrace = Backtrace::force_capture();
        tracing::error!(backtrace = %backtrace, "panic captured backtrace");
    }));

    match std::panic::catch_unwind(|| commands::run_command(seed, options.command)) {
        Ok(output) => match output {
            Ok(_) => {
                tracing::info!(seed = seed, "simulation complete");
                ExitCode::SUCCESS
            }
            Err(err) => {
                tracing::error!(seed = seed, error = %err, "simulation ended with an error");
                ExitCode::FAILURE
            }
        },
        Err(_) => {
            tracing::error!(seed = seed, "simulation ended with a panic");
            ExitCode::FAILURE
        }
    }
}
