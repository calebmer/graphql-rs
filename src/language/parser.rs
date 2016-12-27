use super::lexer;
use super::ast;

pub fn parse<I>(iter: I) -> ast::Document where I: IntoIterator<Item=char> {}
