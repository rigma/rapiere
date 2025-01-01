use crate::cli::Commands;

mod lexer;

pub(crate) fn run_command(command: Commands, seed: u64) {
    match command {
        Commands::Lexer(_) => lexer::entrypoint(seed),
    }
}
