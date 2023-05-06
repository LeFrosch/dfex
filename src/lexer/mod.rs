use crate::ast;
use crate::ast::Tree;

mod scanner;
mod parser;

pub fn parser(text: &str) -> ast::Tree {
    let scanner = scanner::AutoConcatenation::new(scanner::Scanner::new(text));
    let mut parser = parser::Parser::new(scanner);

    let root = parser.parse();

    Tree { root }
}