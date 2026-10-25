use std::rc::Rc;

use crate::{
    loki_core::ast::{
        diag::{Diag, DiagLevel},
        node::{Expr, Ident, IfExpr, VarDeclr},
        span::Location,
        token::TokenKind as TKind,
        types::Type,
        value::Literal,
    },
    symtable::{Mutability, SymInfo, SymbolTable},
};

#[derive(Debug)]
pub struct Analyzer {
    src: String,
    nodes: Rc<[Expr]>,
    scope: SymbolTable,
    infered_ty: Type,
}

impl Analyzer {
    pub fn new(scope: SymbolTable) -> Self {
        Self {
            src: String::new(),
            nodes: Rc::new([]),
            scope,
            infered_ty: Type::Infered,
        }
    }

    pub fn analyze(&mut self, src: String, nodes: Rc<[Expr]>) -> Vec<Diag> {
        self.src = src;
        self.nodes = nodes;
        let mut diags: Vec<Diag> = vec![];
        for node in self.nodes.clone().iter() {
            if let Err(diag) = self.expr(node) {
                diags.push(diag);
            }
        }
        diags
    }

    pub fn expr(&mut self, expr: &Expr) -> Result<Type, Diag> {
        match expr {
            Expr::Literal(literal, ..) => {
                if matches!(literal, Literal::Int(_)) {
                    if self.infered_ty == Type::Infered {
                        return Ok(Type::I32);
                    }
                    return Ok(self.infered_ty.clone());
                }
                Ok(literal.into())
            }
            Expr::Binary(binary) => {
                let lhs = self.expr(binary.lhs())?;
                let rhs = binary.rhs();
                if lhs != self.expr(rhs)? {
                    return Err(Diag::new(
                        DiagLevel::Error("Type of lhs must be same as rhs"),
                        "this is different than rhs type.",
                        rhs.location(),
                    ));
                }
                Ok(lhs)
            }
            Expr::Block(block) => {
                let expressions = block.expression();
                let Some(last) = expressions.last() else {
                    return Ok(Type::Infered);
                };
                let expressions = &expressions[0..expressions.len() - 1];
                for expr in expressions {
                    self.expr(expr)?;
                }
                self.expr(last)
            }
            Expr::Identifier(identifier) => self.ident(identifier),
            Expr::IfExpr(if_expr) => self.if_expr(if_expr),
            Expr::VarDeclr(var_declr) => self.var_declr(var_declr),
            Expr::VarAssign(var_assign) => {
                let vars = var_assign.vars();
                let values = var_assign.values();
                if vars.len() != values.len() && values.len() != 1 {
                    return Err(Self::error(
                        "Unmatched variable group",
                        "expected same or 1 value",
                        values[0].location(),
                    ));
                }
                for i in 0..vars.len() {
                    let Some(var) = self.scope.get(vars[i].name()) else {
                        return Err(Self::error(
                            "Variable never declared",
                            "not found in this scope",
                            vars[i].location(),
                        ));
                    };
                    if var.0.mutability().is_immutable() {
                        dbg!(&var);
                        let mut error = Self::error(
                            "Cannot assign to immutable variable",
                            "variable is immutable",
                            var.0.location(),
                        );
                        error.add_note("Variable is immutable by default, which mean the variable cant change the value.");
                        return Err(error);
                    }
                    self.expr(&values[i])?;
                }

                self.expr(values.last().unwrap())
            }
            Expr::While(while_expr) => {
                self.expr(while_expr.condition())?;
                self.expr(while_expr.body())
            }
            Expr::ForExpr(for_expr) => {
                let ty = self.expr(for_expr.expr())?;
                let pattern = for_expr.pattern();
                // TODO: self.scope.push()
                self.scope.insert_info(
                    pattern.name(),
                    SymInfo::new(ty, pattern.location(), Mutability::Immutable),
                );
                self.expr(for_expr.body())
                // TODO: self.scope.pop()
            }
            Expr::Unary(unary) => {
                let ty = self.expr(unary.expr())?;
                match ty {
                    Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::Usize => {
                        if unary.op() == &TKind::Bang {
                            return Err(Self::error(
                                "Cannot use '!' unary operator to int type",
                                "implicit invertion to bool",
                                unary.location(),
                            ));
                        }
                    }
                    Type::Infered | Type::Unknown => {
                        return Err(Self::error(
                            "Type Error",
                            "Invalid unary expression",
                            unary.location(),
                        ));
                    }
                };
                Ok(ty)
            }
        }
    }

    pub fn if_expr(&mut self, if_expr: &IfExpr) -> Result<Type, Diag> {
        let mut ty = Type::Unknown;

        for (condition, body) in if_expr.cases() {
            self.expr(condition)?;
            if ty.is_unknown() {
                ty = self.expr(body)?;
            } else if self.expr(body)? != ty {
                return Err(Self::error(
                    "Type Error",
                    "type missmatch with previous if statement",
                    body.location(),
                ));
            }
        }

        if let Some(else_body) = if_expr.else_body()
            && self.expr(else_body)? != ty
        {
            return Err(Self::error(
                "Type Error",
                "type missmatch with previous if statement",
                else_body.location(),
            ));
        }
        Ok(ty)
    }

    pub fn var_declr(&mut self, var_declr: &VarDeclr) -> Result<Type, Diag> {
        let ty = match var_declr.ty() {
            Some(id) => id.try_into()?,
            None => Type::Infered,
        };
        self.infered_ty = ty.clone();
        let Some(values) = var_declr.values() else {
            for var in var_declr.vars() {
                self.scope.insert_info(
                    var.name(),
                    SymInfo::new(ty.clone(), var.location(), var_declr.mutability().clone()),
                );
            }
            return Ok(ty);
        };

        let value_ty = self.expr(&values[0])?;
        if ty != value_ty && ty != Type::Infered {
            return Err(Self::error(
                "The value is different than the annotated type type.",
                "",
                values[0].location(),
            ));
        }
        if values.len() == 1 {
            for var in var_declr.vars() {
                self.scope.insert_info(
                    var.name(),
                    SymInfo::new(
                        value_ty.clone(),
                        var.location(),
                        var_declr.mutability().clone(),
                    ),
                );
            }
        }

        let vars = var_declr.vars();
        if values.len() != vars.len() && values.len() != 1 {
            return Err(Self::error(
                "Unmatched variable group",
                "expected same or 1 value",
                values[0].location(),
            ));
        }
        for i in 0..vars.len() {
            let value = self.expr(&values[i])?;
            if value != value_ty {
                return Err(Self::error(
                    "The value is different than the annotated type type.",
                    "",
                    values[0].location(),
                ));
            }
            let var = vars[i].clone();
            self.scope.insert_info(
                var.name(),
                SymInfo::new(
                    value_ty.clone(),
                    var.location(),
                    var_declr.mutability().clone(),
                ),
            );
        }

        Ok(value_ty)
    }

    fn ident(&self, ident: &Ident) -> Result<Type, Diag> {
        let name = ident.name();
        if !self.scope.contain(name) {
            return Err(Self::error(
                "Variable never declared",
                "not found in this scope",
                ident.location(),
            ));
        }
        let ty = self.scope.get(name).unwrap().0.ty().clone();
        Ok(ty)
    }

    const fn error(desc: &'static str, message: &'static str, location: Location) -> Diag {
        Diag::new(DiagLevel::Error(desc), message, location)
    }
}
