use std::fmt::{Debug, Display};

use crate::loki_core::ast::span::Location;

const BOLD: &str = "\x1B[1m";
const RED: &str = "\x1B[1;31m";
const RESET: &str = "\x1B[0m";
const CYAN: &str = "\x1B[96m";
const BRIGHT_CYAN: &str = "\x1B[1;96m";

#[derive(Clone, Copy, Debug)]
pub enum DiagLevel {
    Error(&'static str),
    Warning,
}

impl Display for DiagLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(msg) => write!(f, "{RED}Error: {RESET}{BOLD}{msg}{RESET}"),
            Self::Warning => write!(f, "Warning"),
        }
    }
}

#[derive(Debug)]
pub struct Diag {
    level: DiagLevel,
    message: &'static str,
    location: Location,
    notes: Vec<&'static str>,
}

impl Diag {
    pub const fn new(level: DiagLevel, message: &'static str, location: Location) -> Self {
        Self {
            level,
            message,
            location,
            notes: vec![],
        }
    }

    pub const fn message(&self) -> &'static str {
        self.message
    }
    pub const fn level(&self) -> DiagLevel {
        self.level
    }
    pub const fn location(&self) -> Location {
        self.location
    }
    pub fn notes(&self) -> &[&'static str] {
        &self.notes
    }
    pub fn add_note(&mut self, note: &'static str) {
        self.notes.push(note);
    }
}

pub struct DiagRenderer<'src> {
    src: &'src str,
}

impl<'src> DiagRenderer<'src> {
    pub const fn new(src: &'src str) -> Self {
        Self { src }
    }

    pub fn render(&self, diag: &Diag) {
        let line = diag.location().line();
        let start = diag.location().start();
        let end = diag.location().end();
        let message = diag.message();

        let line_len = line.ilog10() + 2;
        let indent = &" ".repeat((4 - line_len) as usize);
        let line_len = &" ".repeat(line_len as usize);

        let snippet: Vec<&str> = self.src.lines().collect();
        // we need to subtract 1 because
        // vector index start from 0 not 1
        let snippet = snippet[line - 1];

        println!("{} ", diag.level());
        println!("{indent}{line_len}{CYAN}|{RESET}");
        println!("{indent}{CYAN}{line} |{RESET} {snippet}");
        let msg_indent = &" ".repeat(/*until*/ start - 1);
        Self::render_message(indent, line_len, msg_indent, message, start, end);
        println!("{indent}{line_len}{CYAN}|{RESET}");
        Self::render_notes(indent, line_len, diag.notes());
    }

    fn render_message(
        indent: &str,
        line_len: &str,
        msg_indent: &str,
        message: &str,
        start: usize,
        end: usize,
    ) {
        let arrow = if start == end {
            "^"
        } else {
            &"-".repeat(end - start)
        };
        println!(
            "{indent}{line_len}{CYAN}|{RESET} {msg_indent}{BRIGHT_CYAN}{arrow} {message} ({start}:{end}){RESET}"
        );
    }

    fn render_notes(indent: &str, line_len: &str, notes: &[&'static str]) {
        for note in notes {
            println!("{indent}{line_len}{CYAN}={RESET} {BOLD}note:{RESET} {note}");
        }
    }
}
