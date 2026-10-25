use std::rc::Rc;

use crate::loki_core::ast::diag::Diag;
use crate::loki_core::ast::node::{
    Binary, Block, Expr, ForExpr, Ident, IfExpr, Unary, VarAssign, VarDeclr, While,
};
use crate::loki_core::ast::span::Location;
use crate::loki_core::ast::token::{Token, TokenKind as TKind};
use crate::loki_core::ast::value::Literal;
use crate::symtable::Mutability;

use super::diag::DiagLevel;

pub struct Parser<'src> {
    src: &'src str,
    tokens: &'src [Token],
    index: usize,
}

type ParseResult = Result<Expr, Diag>;

impl<'src> Parser<'src> {
    pub const fn new(src: &'src str, tokens: &'src [Token]) -> Self {
        Self {
            src,
            tokens,
            index: 0usize,
        }
    }

    pub fn parse(&mut self) -> Result<Rc<[Expr]>, Rc<[Diag]>> {
        let mut nodes: Vec<Expr> = vec![];
        let mut errors: Vec<Diag> = vec![];

        while let Ok(current) = self.current() {
            if matches!(current.kind(), TKind::Eof) {
                break;
            }

            match self.expr() {
                Ok(expr) => nodes.push(expr),
                Err(err) => {
                    while let Ok(current) = self.current() {
                        if [TKind::Eof].contains(current.kind()) {
                            break;
                        }
                        self.advance();
                    }
                    errors.push(err);
                    continue;
                }
            }

            if let Err(error) = self.consume_end() {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            return Ok(nodes.into());
        }
        Err(errors.into())
    }

    fn consume_end(&mut self) -> Result<(), Diag> {
        let current = self.current()?;
        match current.kind() {
            TKind::Eof | TKind::Newline => {
                self.consume_newline();
                Ok(())
            }
            _ => Err(self.error("Expected EOF", "")),
        }
    }

    fn consume_newline(&mut self) {
        loop {
            if !self.matched(&TKind::Newline) {
                break;
            }
            self.advance();
        }
    }

    fn expr_list(&mut self) -> Result<Rc<[Expr]>, Diag> {
        let mut expr_list = vec![self.expr()?];

        while let Ok(current) = self.current() {
            if current.kind() != &TKind::Comma {
                break;
            }
            self.advance(); // ,

            expr_list.push(self.expr()?);
        }

        Ok(expr_list.into())
    }

    fn expr(&mut self) -> ParseResult {
        let current = self.current()?;
        match current.kind() {
            TKind::KeywordIf => self.if_expr(),
            TKind::KeywordDo => self.block(),
            TKind::KeywordWhile => self.while_loop(),
            TKind::KeywordMut => self.var_declaration(),
            TKind::KeywordFor => self.for_loop(),
            TKind::Identifier(_) => self.ident_branch(),
            _ if self.current_is_ident_list() => self.ident_branch(),
            _ => self.logical(),
        }
    }

    fn block(&mut self) -> ParseResult {
        self.advance(); // do
        let mut expressions: Vec<Expr> = vec![];
        self.consume_newline();
        while let Ok(current) = self.current() {
            if current.kind() == &TKind::KeywordEnd || current.kind() == &TKind::Eof {
                break;
            }
            expressions.push(self.expr()?);
            self.consume_newline();
        }
        self.expect(&TKind::KeywordEnd, "Expected end keyword")?;
        Ok(Expr::Block(Block::new(expressions.into())))
    }

    fn for_loop(&mut self) -> ParseResult {
        let start = self.current()?.location().start();

        self.advance(); // for
        if !self.matched(&TKind::Lparen) {
            let mut error = self.error("Unexpected token", "Expected a condition");
            if !self.matched(&TKind::Eof) {
                error.add_note("'If condition' must wrapped by parentheses '()'");
            }
            return Err(error);
        }
        self.advance();
        let pattern = self.identifier()?;
        self.expect(&TKind::KeywordIn, "Expected 'in' keyword ")?;
        let expr = self.expr()?;
        self.expect(&TKind::Rparen, "Expected Rparen ')'")?;
        let body = self.expr()?;

        let end = self.current()?.location();
        let location = Location::new(start, end.end(), end.line());
        Ok(Expr::ForExpr(ForExpr::new(location, pattern, expr, body)))
    }

    fn while_loop(&mut self) -> ParseResult {
        let start = self.current()?.location().start();
        self.advance(); // while
        if !self.matched(&TKind::Lparen) {
            let mut error = self.error("Unexpected token", "Expected a condition");
            if !self.matched(&TKind::Eof) {
                error.add_note("'If condition' must wrapped by parentheses '()'");
            }
            return Err(error);
        }
        self.advance();
        let condition = self.expr()?;
        self.expect(&TKind::Rparen, "Expected Rparen ')' ")?;

        let body = self.expr()?;
        let end = self.current()?.location();
        let location = Location::new(start, end.end(), end.line());
        Ok(Expr::While(While::new(condition, body, location)))
    }

    fn if_expr(&mut self) -> ParseResult {
        let mut cases = vec![self.if_cases()?];

        let else_body = loop {
            if !self.matched(&TKind::KeywordElse) {
                break None;
            }

            self.advance(); // else

            if !self.matched(&TKind::KeywordIf) {
                break Some(self.expr()?);
            }
            cases.push(self.if_cases()?);
        };

        Ok(Expr::IfExpr(IfExpr::new(cases.into(), else_body)))
    }

    fn if_cases(&mut self) -> Result<(Expr, Expr), Diag> {
        self.advance(); // if

        if !self.matched(&TKind::Lparen) {
            let mut error = self.error("Unexpected token", "Expected a condition");
            if !self.matched(&TKind::Eof) {
                error.add_note("'If condition' must wrapped by parentheses '()'");
            }
            return Err(error);
        }

        self.advance(); // (
        let condition = self.expr()?;
        self.expect(&TKind::Rparen, "Expected Rparen ')' ")?;

        let body = self.if_body()?;

        Ok((condition, body))
    }

    fn if_body(&mut self) -> Result<Expr, Diag> {
        Ok(if self.matched(&TKind::KeywordDo) {
            self.advance();
            let mut expressions: Vec<Expr> = vec![];
            while let Ok(current) = self.current() {
                match current.kind() {
                    TKind::KeywordElse => break,
                    TKind::KeywordEnd => {
                        self.advance();
                        break;
                    }
                    _ => expressions.push(self.expr()?),
                }
            }
            Expr::Block(Block::new(expressions.into()))
        } else {
            self.expr()?
        })
    }

    fn ident_branch(&mut self) -> ParseResult {
        // FIXME: now it so bad, please refactor it.
        let mut idx_until_not_ident_or_comma = 0;
        let next = loop {
            let current = self.peek(idx_until_not_ident_or_comma)?;
            if !matches!(current.kind(), TKind::Identifier(_)) && current.kind() != &TKind::Comma {
                break current;
            }
            idx_until_not_ident_or_comma += 1;
        };

        match next.kind() {
            TKind::Colon => self.var_declaration(),
            TKind::Equal => self.var_assign(),
            _ => self.logical(),
        }
    }

    fn var_declaration(&mut self) -> ParseResult {
        let mutability = if self.matched(&TKind::KeywordMut) {
            self.advance();
            Mutability::Mutable
        } else {
            Mutability::Immutable
        };

        let vars = self.ident_list();
        self.expect(&TKind::Colon, "Expected colon ':'")?;

        let ty = if self.matched(&TKind::Equal) {
            None
        } else {
            Some(self.identifier()?)
        };

        let value = if self.matched(&TKind::Equal) {
            self.advance();
            Some(self.expr_list()?)
        } else {
            None
        };

        Ok(Expr::VarDeclr(VarDeclr::new(mutability, vars?, ty, value)))
    }

    fn var_assign(&mut self) -> ParseResult {
        let vars = self.ident_list();
        self.advance(); // NOTE: we already check the = before
        let values = self.expr_list();
        Ok(Expr::VarAssign(VarAssign::new(vars?, values?)))
    }

    fn ident_list(&mut self) -> Result<Rc<[Ident]>, Diag> {
        let mut ident_list = vec![self.identifier()?];

        while let Ok(current) = self.current() {
            if current.kind() != &TKind::Comma {
                break;
            }
            self.advance(); // ,

            ident_list.push(self.identifier()?);
        }

        Ok(ident_list.into())
    }

    fn current_is_ident_list(&self) -> bool {
        let Ok(current) = self.current_kind() else {
            return false;
        };
        let Ok(next) = self.next() else {
            return false;
        };
        matches!(current, &TKind::Identifier(_)) && next.kind() == &TKind::Comma
    }

    fn logical(&mut self) -> ParseResult {
        self.binary(Self::comparative, &[TKind::DoubleAnd, TKind::DoublePipe])
    }

    fn comparative(&mut self) -> ParseResult {
        self.binary(
            Self::sum,
            &[
                TKind::Less,
                TKind::LessEq,
                TKind::Greater,
                TKind::GreaterEq,
                TKind::DoubleEq,
            ],
        )
    }

    fn sum(&mut self) -> ParseResult {
        self.binary(Self::product, &[TKind::Plus, TKind::Minus])
    }

    fn product(&mut self) -> ParseResult {
        self.binary(Self::unary, &[TKind::Slash, TKind::Star, TKind::Percent])
    }

    fn unary(&mut self) -> ParseResult {
        let op = self.current()?;
        match op.kind() {
            TKind::Bang | TKind::Minus | TKind::Plus => {
                let start = op.location().start();
                let op = op.kind().clone();
                self.advance();
                let expr = self.unary()?;
                let end = expr.location();
                let location = Location::new(start, end.end(), end.line());
                Ok(Expr::Unary(Unary::new(location, op, expr)))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> ParseResult {
        let current = self.current()?;
        match current.kind() {
            TKind::Int(span) => {
                let int = Literal::Int(span.to_str(self.src).to_owned());
                let location = *current.location();
                self.advance();
                Ok(Expr::Literal(int, location))
            }
            TKind::Identifier(_) => Ok(Expr::Identifier(self.identifier()?)),
            _ => Err(self.error(
                "Unexpected token",
                "Expected expression, literal or identifier.",
            )),
        }
    }

    fn identifier(&mut self) -> Result<Ident, Diag> {
        let current = self.current()?;
        let lexeme = current.lexeme(self.src);
        let ident = Ident::new(lexeme.into(), *current.location());
        self.advance();
        Ok(ident)
    }

    fn binary<F>(&mut self, mut func: F, op: &[TKind]) -> ParseResult
    where
        F: FnMut(&mut Self) -> ParseResult,
    {
        let mut lhs = func(self)?;
        loop {
            let current = self.current()?;
            if !op.contains(current.kind()) {
                break;
            }
            let op = current.kind().clone();
            self.advance(); // op

            let rhs = func(self)?;
            lhs = Expr::Binary(Binary::new(lhs, op, rhs));
        }
        Ok(lhs)
    }

    #[inline]
    fn current(&self) -> Result<&Token, Diag> {
        if self.index >= self.tokens.len() {
            return Err(Self::error_with_location(
                "Unexpected EOF",
                "",
                *self.previous_location(),
            ));
        }
        Ok(&self.tokens[self.index])
    }

    fn previous_location(&self) -> &Location {
        let target = self.index.saturating_sub(1);
        self.tokens[target].location()
    }

    #[inline]
    fn next(&self) -> Result<&Token, Diag> {
        self.peek(1)
    }

    #[inline]
    fn peek(&self, n: usize) -> Result<&Token, Diag> {
        let target = self.index.saturating_add(n);
        if target >= self.tokens.len() {
            return Err(self.error("Unexpected EOF", ""));
        }
        Ok(&self.tokens[target])
    }

    #[inline]
    fn current_kind(&self) -> Result<&TKind, Diag> {
        Ok(self.current()?.kind())
    }

    #[inline]
    fn matched(&self, target: &TKind) -> bool {
        let Ok(current) = self.current() else {
            return false;
        };
        current.kind() == target
    }

    #[inline]
    const fn advance(&mut self) {
        self.index = self.index.strict_add(1);
    }

    #[inline]
    fn expect(&mut self, target: &TKind, message: &'static str) -> Result<(), Diag> {
        if self.current()?.kind() == target {
            self.advance();
            return Ok(());
        }

        Err(self.error("Unexpected Token", message))
    }

    #[inline]
    fn error(&self, desc: &'static str, message: &'static str) -> Diag {
        let location = *self.current().unwrap().location();
        Diag::new(DiagLevel::Error(desc), message, location)
    }

    const fn error_with_location(
        desc: &'static str,
        message: &'static str,
        location: Location,
    ) -> Diag {
        Diag::new(DiagLevel::Error(desc), message, location)
    }
}
