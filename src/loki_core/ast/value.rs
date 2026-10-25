#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    // for unknown size int
    Int(String),
    Usize(usize),
}
