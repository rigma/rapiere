use crate::{cli::CommandArgs, generation::Arbitrary, models::lexer::RawInput};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rapiere_lexer::{Error, Lexer};
use std::{fs::File, io::Write};

pub(crate) fn entrypoint(seed: u64, args: CommandArgs) -> Result<(), Error> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    tracing::info!("generating raw input");
    let raw_input = RawInput::arbitrary(&mut rng);

    tracing::info!("generation done");
    tracing::debug!(raw = %raw_input, "generated raw input");

    let raw_input = raw_input.as_bytes();
    let mut scanner = Lexer::new(&raw_input);

    let mut file = File::create(args.plan_path()).expect("unable to save raw input");
    file.write_all(&raw_input)
        .expect("unable to save raw input");

    loop {
        if let Some(token) = scanner.next_token()? {
            tracing::trace!(token = %token, "token scanned")
        } else {
            break;
        }
    }

    Ok(())
}
