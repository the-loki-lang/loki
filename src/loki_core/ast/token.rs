use crate::loki_core::ast::span::{Location, Span};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // !
    Bang,
    // &&
    DoubleAnd,
    // ||
    DoublePipe,
    // :
    Colon,
    //,
    Comma,
    // ==
    DoubleEq,
    // >=
    GreaterEq,
    // <=
    LessEq,
    // >
    Greater,
    // <
    Less,
    Eof,
    // =
    Equal,
    Float(Span),
    Identifier(Span),
    Int(Span),
    // -
    Minus,
    // \n ;
    Newline,
    // !=
    NotEqual,
    // %
    Percent,
    // +
    Plus,
    // /
    Slash,
    // *
    Star,

    Lparen,
    Rparen,

    KeywordIf,
    KeywordElse,
    KeywordDo,
    KeywordEnd,
    KeywordWhile,
    KeywordMut,
    KeywordFunc,
    KeywordFor,
    KeywordIn,
}

impl TokenKind {
    pub fn lexeme<'src>(&'src self, src: &'src str) -> &'src str {
        match self {
            Self::Float(span) | Self::Int(span) | Self::Identifier(span) => span.to_str(src),

            Self::Bang => "!",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Eof => "EOF",
            Self::Equal => "=",
            Self::Minus => "-",
            Self::Percent => "%",
            Self::Plus => "+",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Newline => "newline or semicolon ';'",

            Self::DoubleEq => "==",
            Self::NotEqual => "!=",
            Self::GreaterEq => ">=",
            Self::LessEq => "<=",
            Self::Less => "<",
            Self::Greater => ">",

            Self::DoubleAnd => "&&",
            Self::DoublePipe => "||",

            Self::Lparen => "(",
            Self::Rparen => ")",

            Self::KeywordIf => "if",
            Self::KeywordIn => "in",
            Self::KeywordElse => "else",
            Self::KeywordDo => "do",
            Self::KeywordEnd => "end",
            Self::KeywordFunc => "func",
            Self::KeywordMut => "mut",
            Self::KeywordWhile => "while",
            Self::KeywordFor => "for",
        }
    }

    /// Returns `true` if the token kind is [`Identifier`].
    ///
    /// [`Identifier`]: TokenKind::Identifier
    #[must_use]
    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(..))
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    location: Location,
}

impl Token {
    pub fn new(kind: TokenKind, location: Location) -> Self {
        Self { kind, location }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn lexeme<'src>(&'src self, src: &'src str) -> &'src str {
        self.kind.lexeme(src)
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}
