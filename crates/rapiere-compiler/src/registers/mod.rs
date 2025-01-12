pub use literal::Literal;
use register::Register;

mod literal;
mod register;

pub type Index = u8;
pub type LiteralRegister = Register<Literal>;
pub type PathRegister = Register<String>;
