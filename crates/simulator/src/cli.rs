use clap::{command, Args, Parser, Subcommand};
use rand::RngCore;

#[derive(Parser)]
#[command(name = "simulator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    #[inline(always)]
    pub fn seed(&self) -> u64 {
        match &self.command {
            Commands::Lexer(args) => {
                if let Some(seed) = &args.seed {
                    *seed
                } else {
                    rand::thread_rng().next_u64()
                }
            }
        }
    }
}

#[derive(Args)]
pub struct CommandArgs {
    #[arg(help = "provide a seed to reproduce a run")]
    pub seed: Option<u64>,
}

#[derive(Subcommand)]
pub enum Commands {
    Lexer(CommandArgs),
}
