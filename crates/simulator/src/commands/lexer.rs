use crate::{generation::Arbitrary, models::lexer::RawInput};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rapiere_lexer::Lexer;
use std::{fs::File, io::Write};

pub(crate) fn entrypoint(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    tracing::info!("generating raw input");
    let raw_input = RawInput::arbitrary(&mut rng);

    tracing::info!("generation done");
    tracing::debug!(raw = %raw_input, "generated raw input");

    let raw_input = raw_input.as_bytes();
    let mut scanner = Lexer::new(&raw_input);

    let mut file = File::create("raw_input.txt").expect("unable to save raw input");
    file.write_all(&raw_input)
        .expect("unable to save raw input");

    loop {
        match scanner.next_token() {
            Ok(token) => {
                if token.is_none() {
                    break;
                }
            }
            Err(err) => {
                tracing::error!(error = %err, "unable to get next token");
                break;
            }
        }
    }
}
