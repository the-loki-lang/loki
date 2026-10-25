use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::loki_core::ast::span::Location;
use crate::loki_core::ast::types::Type;
use crate::loki_core::ast::value::Literal;

#[derive(Debug)]
pub enum Symbol {
    Undeclared,
    Literal(Literal),
}

impl Default for Symbol {
    fn default() -> Self {
        Self::Undeclared
    }
}

impl Symbol {
    /// Returns `true` if the symbol is [`Literal`].
    ///
    /// [`Literal`]: Symbol::Literal
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    pub fn try_into_literal(self) -> Result<Literal, Self> {
        if let Self::Literal(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl Mutability {
    /// Returns `true` if the mutability is [`Mutable`].
    ///
    /// [`Mutable`]: Mutability::Mutable
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        matches!(self, Self::Mutable)
    }

    /// Returns `true` if the mutability is [`Immutable`].
    ///
    /// [`Immutable`]: Mutability::Immutable
    #[must_use]
    pub fn is_immutable(&self) -> bool {
        matches!(self, Self::Immutable)
    }
}

#[derive(Debug)]
pub struct SymInfo {
    ty: Type,
    location: Location,
    mutability: Mutability,
}

impl SymInfo {
    pub fn new(ty: Type, location: Location, mutability: Mutability) -> Self {
        Self {
            ty,
            location,
            mutability,
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn mutability(&self) -> &Mutability {
        &self.mutability
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, (SymInfo, Symbol)>,
    parent: Option<Rc<RefCell<Self>>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
        }
    }

    pub fn insert_info<K: Into<String>>(&mut self, k: K, v: SymInfo) {
        self.symbols.insert(k.into(), (v, Symbol::default()));
    }

    pub fn get(&self, k: &str) -> Option<&(SymInfo, Symbol)> {
        self.symbols.get(k)
    }

    pub(crate) fn contain(&self, key: &str) -> bool {
        self.symbols.contains_key(key)
    }
}
