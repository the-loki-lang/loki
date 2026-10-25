use std::io::Write as _;

use crate::{
    loki_core::{
        analyzer::Analyzer,
        ast::{diag::DiagRenderer, lexer::Lexer, parser::Parser},
    },
    symtable::SymbolTable,
};

mod loki_core;
mod symtable;

fn main() {
    repl();
}

fn repl() -> ! {
    let mut src = String::new();
    let scope = SymbolTable::new(None);
    let mut analyzer = Analyzer::new(scope);
    loop {
        src.clear();
        print!("<loki> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut src).unwrap();
        let src = src.trim_end();

        let err_renderer = DiagRenderer::new(src);

        let mut lexer = Lexer::new(src);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => {
                #[cfg(feature = "lexer_dbg")]
                dbg!(&tokens);
                tokens
            }
            Err(diags) => {
                diags.iter().for_each(|d| err_renderer.render(d));
                continue;
            }
        };

        let mut parser = Parser::new(src, &tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(diags) => {
                diags.iter().for_each(|diag| err_renderer.render(diag));
                continue;
            }
        };

        #[cfg(feature = "parser_dbg")]
        dbg!(&ast);

        let err = analyzer.analyze(src.to_owned(), ast);
        if !err.is_empty() {
            err.iter().for_each(|diag| err_renderer.render(diag));
            continue;
        }
    }
}
