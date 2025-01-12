use crate::registers::{LiteralRegister, PathRegister};

#[derive(Clone, Debug, Default)]
pub struct Filter {
    literals: LiteralRegister,
    paths: PathRegister,
}

impl Filter {
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }
}
