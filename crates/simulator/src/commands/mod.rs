use crate::cli::Commands;
use rapiere_lexer::Error;

mod lexer;

pub(crate) fn run_command(seed: u64, command: Commands) -> Result<(), Error> {
    match command {
        Commands::Lexer(args) => {
            tracing::info!("running rapiere-lexer simulation");
            lexer::entrypoint(seed, args)
        }
    }
}
