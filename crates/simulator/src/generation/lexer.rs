use super::{gen_random_string, pick, readable_name_custom, Arbitrary};
use crate::models::lexer::{Fragment, RawInput};
use rand::Rng;

enum LiteralType {
    Boolean,
    Float,
    Integer,
    Null,
    String,
}

enum FragmentKind {
    Field,
    Keyword,
    Literal,
    Operator,
    Other,
    Whitespace,
}

impl<'s> Arbitrary for Fragment<'s> {
    fn arbitrary<R: Rng>(rng: &mut R) -> Self {
        let ty = pick(
            &[
                FragmentKind::Field,
                FragmentKind::Keyword,
                FragmentKind::Literal,
                FragmentKind::Operator,
                FragmentKind::Other,
                FragmentKind::Whitespace,
            ],
            rng,
        );

        match ty {
            FragmentKind::Field => gen_field(rng),
            FragmentKind::Keyword => Self::new(*pick(&["AND", "OR", "NOT"], rng)),
            FragmentKind::Literal => gen_literal(rng),
            FragmentKind::Operator => gen_operator(rng),
            FragmentKind::Other => Self::new(*pick(&["(", ")", ",", "."], rng)),
            FragmentKind::Whitespace => {
                let whitespace = vec![*pick(&[0x9, 0xa, 0xc, b'\n', b' '], rng)];
                let whitespace =
                    String::from_utf8(whitespace).expect("unable to generate valid whitespace");

                Self::new(&whitespace)
            }
        }
    }
}

impl<'i> Arbitrary for RawInput<'i> {
    fn arbitrary<R: Rng>(rng: &mut R) -> Self {
        let size = rng.gen_range(64..2 * 1024);
        let mut fragments = Vec::with_capacity(size);

        for _ in 0..size {
            fragments.push(Fragment::arbitrary(rng));
        }

        Self::new(fragments)
    }
}

#[inline(always)]
fn gen_field<'s, R: Rng>(rng: &mut R) -> Fragment<'s> {
    Fragment::new(&readable_name_custom("_", rng))
}

#[inline(always)]
fn gen_literal<'s, R: Rng>(rng: &mut R) -> Fragment<'s> {
    let ty = pick(
        &[
            LiteralType::Boolean,
            LiteralType::Float,
            LiteralType::Integer,
            LiteralType::Null,
            LiteralType::String,
        ],
        rng,
    );

    match ty {
        LiteralType::Boolean => Fragment::new(*pick(&["true", "false"], rng)),
        LiteralType::Float => {
            let value = rng.gen::<f32>();
            let value = if rng.gen() { -value } else { value };

            Fragment::new(&value.to_string())
        }
        LiteralType::Integer => Fragment::new(&rng.gen::<i64>().to_string()),
        LiteralType::Null => Fragment::new("null"),
        LiteralType::String => Fragment::new(&gen_random_string(rng)),
    }
}

#[inline(always)]
fn gen_operator<'s, R: Rng>(rng: &mut R) -> Fragment<'s> {
    Fragment::new(*pick(&[":", "-", "=", "!=", ">", ">=", "<", "<="], rng))
}
