use std::rc::Rc;

use crate::{
    loki_core::ast::{span::Location, token::TokenKind, value::Literal},
    symtable::Mutability,
};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal, Location),
    Binary(Binary),
    Block(Block),
    Identifier(Ident),
    IfExpr(IfExpr),
    ForExpr(ForExpr),
    Unary(Unary),
    VarDeclr(VarDeclr),
    VarAssign(VarAssign),
    While(While),
}

impl Expr {
    pub fn location(&self) -> Location {
        match self {
            Expr::Literal(.., location) => *location,
            Expr::Binary(binary) => todo!(),
            Expr::Block(block) => todo!(),
            Expr::Identifier(identifier) => todo!(),
            Expr::IfExpr(if_expr) => todo!(),
            Expr::ForExpr(for_expr) => for_expr.location,
            Expr::Unary(unary) => unary.location,
            Expr::VarDeclr(var_declr) => todo!(),
            Expr::VarAssign(var_assign) => todo!(),
            Expr::While(while_loop) => while_loop.location,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForExpr {
    location: Location,
    pattern: Ident,
    expr: Box<Expr>,
    body: Box<Expr>,
}

impl ForExpr {
    pub fn new(location: Location, pattern: Ident, expr: Expr, body: Expr) -> Self {
        Self {
            location,
            pattern,
            expr: Box::new(expr),
            body: Box::new(body),
        }
    }

    pub fn pattern(&self) -> &Ident {
        &self.pattern
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn body(&self) -> &Expr {
        &self.body
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Debug)]
pub struct While {
    condition: Box<Expr>,
    body: Box<Expr>,
    location: Location,
}

impl While {
    pub fn new(condition: Expr, body: Expr, location: Location) -> Self {
        Self {
            condition: Box::new(condition),
            body: Box::new(body),
            location,
        }
    }

    pub fn condition(&self) -> &Expr {
        &self.condition
    }

    pub fn body(&self) -> &Expr {
        &self.body
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    expression: Rc<[Expr]>,
}

impl Block {
    pub fn new(expression: Rc<[Expr]>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &[Expr] {
        &self.expression
    }

    pub fn last_expr(&self) -> Option<&Expr> {
        self.expression.last()
    }
}

#[derive(Clone, Debug)]
pub struct IfExpr {
    cases: Rc<[(Expr, Expr)]>,
    else_body: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn new(cases: Rc<[(Expr, Expr)]>, else_body: Option<Expr>) -> Self {
        Self {
            cases,
            else_body: else_body.map(Box::new),
        }
    }

    pub fn cases(&self) -> &[(Expr, Expr)] {
        &self.cases
    }

    pub fn else_body(&self) -> Option<&Box<Expr>> {
        self.else_body.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct VarAssign {
    vars: Rc<[Ident]>,
    values: Rc<[Expr]>,
}

impl VarAssign {
    /// Creates a new [`VarAssign`].
    #[inline]
    pub const fn new(vars: Rc<[Ident]>, values: Rc<[Expr]>) -> Self {
        Self { vars, values }
    }

    /// Returns a reference to the vars of this [`VarAssign`].
    #[inline]
    pub fn vars(&self) -> &[Ident] {
        &self.vars
    }

    /// Returns a reference to the values of this [`VarAssign`].
    #[inline]
    pub fn values(&self) -> &[Expr] {
        &self.values
    }
}

#[derive(Clone, Debug)]
pub struct VarDeclr {
    mutability: Mutability,
    vars: Rc<[Ident]>,
    ty: Option<Box<Ident>>,
    values: Option<Rc<[Expr]>>,
}

impl VarDeclr {
    pub fn new(
        mutability: Mutability,
        vars: Rc<[Ident]>,
        ty: Option<Ident>,
        values: Option<Rc<[Expr]>>,
    ) -> Self {
        Self {
            mutability,
            vars,
            ty: ty.map(Box::new),
            values,
        }
    }

    pub fn vars(&self) -> &[Ident] {
        &self.vars
    }

    pub fn ty(&self) -> Option<&Ident> {
        self.ty.as_deref()
    }

    pub fn values(&self) -> Option<&[Expr]> {
        self.values.as_deref()
    }

    pub fn mutability(&self) -> &Mutability {
        &self.mutability
    }
}

#[derive(Clone, Debug)]
pub struct Binary {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: TokenKind,
}

impl Binary {
    pub fn new(lhs: Expr, op: TokenKind, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        }
    }

    pub fn lhs(&self) -> &Expr {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expr {
        &self.rhs
    }

    pub fn op(&self) -> &TokenKind {
        &self.op
    }
}

#[derive(Clone, Debug)]
pub struct Unary {
    location: Location,
    op: TokenKind,
    expr: Box<Expr>,
}

impl Unary {
    pub fn new(location: Location, op: TokenKind, expr: Expr) -> Self {
        Self {
            location,
            op,
            expr: Box::new(expr),
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn op(&self) -> &TokenKind {
        &self.op
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

#[derive(Clone, Debug)]
pub struct Ident {
    name: Rc<str>,
    location: Location,
}

impl Ident {
    pub fn new(name: Rc<str>, location: Location) -> Self {
        Self { name, location }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn location(&self) -> Location {
        self.location
    }
}
