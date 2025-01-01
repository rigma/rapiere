use anarchist_readable_name_generator_lib::readable_name_custom;
use rand::Rng;

pub mod lexer;

pub trait Arbitrary {
    fn arbitrary<R: Rng>(rng: &mut R) -> Self;
}

pub(crate) fn gen_random_string<R: Rng>(rng: &mut R) -> String {
    let size = rng.gen_range(2..=256);
    let mut output = "\"".to_owned();

    for _ in 0..size {
        let upper_case = rng.gen();
        let byte = if upper_case {
            rng.gen_range(0..26) + b'A'
        } else {
            rng.gen_range(0..26) + b'a'
        };

        output.push(byte as char);
    }

    output.push('"');
    output
}

pub(crate) fn pick<'a, T, R>(choices: &'a [T], rng: &mut R) -> &'a T
where
    R: Rng,
{
    let idx = rng.gen_range(0..choices.len());

    &choices[idx]
}
