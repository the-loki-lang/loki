use std::rc::Rc;

use crate::loki_core::ast::{
    diag::Diag,
    span::{Location, Span},
    token::{Token, TokenKind as Kind},
};

use super::diag::DiagLevel;

pub struct Lexer<'src> {
    src: &'src str,
    index: usize,
    location: Location,
    diags: Vec<Diag>,
}
impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            index: 0usize,
            location: Location::new(1, 1, 1),
            diags: vec![],
        }
    }
    pub fn tokenize(&'src mut self) -> Result<Rc<[Token]>, &'src [Diag]> {
        let mut tokens: Vec<Token> = vec![];
        while let Some(current) = self.current() {
            tokens.push(match current {
                '+' => Token::new(Kind::Plus, self.location),
                '-' => Token::new(Kind::Minus, self.location),
                '*' => Token::new(Kind::Star, self.location),
                '/' => Token::new(Kind::Slash, self.location),
                '%' => Token::new(Kind::Percent, self.location),
                ':' => Token::new(Kind::Colon, self.location),
                ',' => Token::new(Kind::Comma, self.location),
                '(' => Token::new(Kind::Lparen, self.location),
                ')' => Token::new(Kind::Rparen, self.location),
                '=' | '<' | '>' | '&' | '|' | '!' => match self.scan_potential(current) {
                    Some(value) => value,
                    None => break,
                },
                '\n' | ';' => Token::new(Kind::Newline, self.location),
                _ if current.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ if current.is_ascii_digit() => self.scan_digits(),
                _ if current.is_alphanumeric() || current == '_' => self.scan_ident(),
                _ => {
                    self.error("Illegal Character", "Found this");
                    self.advance();
                    continue;
                }
            });

            self.advance();
        }
        tokens.push(Token::new(Kind::Eof, self.location));

        if self.diags.is_empty() {
            return Ok(tokens.into());
        }
        Err(&self.diags)
    }

    fn scan_potential(&mut self, current: char) -> Option<Token> {
        let start = self.location.start();
        let next = self.peek()?;
        let kind = match (current, next) {
            ('=', '=') => {
                self.advance();
                Kind::DoubleEq
            }
            ('!', '=') => {
                self.advance();
                Kind::NotEqual
            }
            ('>', '=') => {
                self.advance();
                Kind::GreaterEq
            }
            ('<', '=') => {
                self.advance();
                Kind::LessEq
            }
            ('&', '&') => {
                self.advance();
                Kind::DoubleAnd
            }
            ('|', '|') => {
                self.advance();
                Kind::DoublePipe
            }
            ('=', _) => Kind::Equal,
            ('!', _) => Kind::Bang,
            ('>', _) => Kind::Greater,
            ('<', _) => Kind::Less,
            _ => unimplemented!(),
        };
        self.advance();
        let location = Location::new(start, self.location.end(), self.location.line());
        Some(Token::new(kind, location))
    }

    fn scan_ident(&mut self) -> Token {
        let start = self.index;
        let location_start = self.location.start();
        while let Some(next) = self.peek() {
            let next_is_not_ident = !next.is_alphanumeric() && next != '_';

            if next_is_not_ident {
                break;
            }
            self.advance();
        }
        let span = Span::new(start, self.index + 1);
        let location = Location::new(
            location_start,
            self.location.end() + 1,
            self.location.line(),
        );
        Token::new(
            match span.to_str(self.src) {
                "do" => Kind::KeywordDo,
                "else" => Kind::KeywordElse,
                "end" => Kind::KeywordEnd,
                "for" => Kind::KeywordFor,
                "if" => Kind::KeywordIf,
                "in" => Kind::KeywordIn,
                "mut" => Kind::KeywordMut,
                "while" => Kind::KeywordWhile,
                _ => Kind::Identifier(span),
            },
            location,
        )
    }

    fn scan_digits(&mut self) -> Token {
        let start = self.index;
        let location_start = self.location.start();
        let mut is_float = false;
        while let Some(next) = self.peek() {
            if next == '.' {
                if is_float {
                    break;
                }
                is_float = true;
                self.advance(); // current
            }
            let next_is_not_digit = !next.is_ascii_digit() && next != '_';
            if next_is_not_digit {
                break;
            }
            self.advance();
        }
        let span = Span::new(start, self.index + 1);
        let location = Location::new(
            location_start,
            self.location.end() + 1,
            self.location.line(),
        );
        if is_float {
            Token::new(Kind::Float(span), location)
        } else {
            Token::new(Kind::Int(span), location)
        }
    }

    fn error(&mut self, desc: &'static str, message: &'static str) {
        self.diags
            .push(Diag::new(DiagLevel::Error(desc), message, self.location));
    }

    #[inline]
    fn advance(&mut self) {
        let Some(current) = self.current() else {
            return;
        };
        self.index = self.index.strict_add(current.len_utf8());
        self.location.advance(current);
    }

    #[inline]
    fn current(&self) -> Option<char> {
        self.src[self.index..].chars().next()
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.src[self.index + 1..].chars().next()
    }
}
