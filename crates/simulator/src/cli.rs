use clap::{command, Args, Parser, Subcommand};
use rand::RngCore;
use std::{borrow::Cow, path::PathBuf};

#[derive(Parser)]
#[command(name = "simulator", about, propagate_version = true, version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

impl Cli {
    #[inline(always)]
    pub(crate) fn seed(&self) -> u64 {
        match &self.command {
            Commands::Lexer(args) => args.seed(),
        }
    }
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Run the simulator to test `rapiere-lexer` crate
    Lexer(CommandArgs),
}

#[derive(Args)]
pub(crate) struct CommandArgs {
    #[arg(help = "Seed of the run to reproduce")]
    seed: Option<u64>,

    #[arg(
        short = 'p',
        long = "plan",
        help = "Path used to save the simulation plan"
    )]
    plan_path: Option<Cow<'static, str>>,
}

impl CommandArgs {
    #[inline(always)]
    fn seed(&self) -> u64 {
        if let Some(seed) = &self.seed {
            *seed
        } else {
            rand::rng().next_u64()
        }
    }

    #[inline(always)]
    pub(crate) fn plan_path(&self) -> PathBuf {
        if let Some(path) = &self.plan_path {
            PathBuf::from(path.as_ref())
        } else {
            PathBuf::from("plan.txt")
        }
    }
}
