mod annotate;
mod ast;
mod error;
mod format;
mod lexer;
mod parser;

pub use crate::error::Error;

pub fn format_acsl_file(input: &str) -> String {
    annotate::format_acsl_annotations(input)
}

pub fn format_acsl_annotation(content: &str) -> String {
    annotate::format_annotation_content(content)
}

pub fn format_expression(expr: &str) -> Result<String, Error> {
    let parsed = parser::Parser::new(expr)?.parse_expression()?;
    Ok(format::format_expr(&parsed))
}
