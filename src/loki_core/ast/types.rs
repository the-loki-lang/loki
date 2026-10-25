use std::collections::HashMap;

use crate::loki_core::ast::{
    diag::{Diag, DiagLevel},
    node::Ident,
    value::Literal,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Infered,
    I8,
    I16,
    I32,
    I64,
    I128,
    Usize,
    Unknown,
    Bool,
    Nil,
    Func {
        params: HashMap<String, Self>,
        return_ty: Box<Self>,
    },
}

impl Type {
    /// Returns `true` if the type is [`Unknown`].
    ///
    /// [`Unknown`]: Type::Unknown
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl From<&Literal> for Type {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::I8(_) => Self::I8,
            Literal::I16(_) => Self::I16,
            Literal::I64(_) => Self::I64,
            Literal::I128(_) => Self::I128,
            Literal::Usize(_) => Self::Usize,
            Literal::I32(_) | Literal::Int(_) => Self::I32,
        }
    }
}

impl TryFrom<&Ident> for Type {
    type Error = Diag;
    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value.name() {
            "bool" => Ok(Self::Bool),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "i128" => Ok(Self::I128),
            "int" => Err(Diag::new(
                DiagLevel::Error("Cannot use int as type, use i8, i16, i32, i64, i128, instead"),
                "replace this with one of those",
                value.location(),
            )),
            "uz" => Ok(Self::Usize),
            _ => Err(Diag::new(
                DiagLevel::Error("Cannot find builtin type"),
                "",
                value.location(),
            )),
        }
    }
}
