#[derive(Clone, Debug, Default, PartialEq)]
pub enum Literal {
    Boolean(bool),

    Float(f32),

    Integer(i64),

    String(String),

    #[default]
    Null,
}
