mod annotate;
mod ast;
mod error;
mod format;
mod parser;

pub use crate::error::Error;

pub fn format_acsl_file(input: &str) -> String {
    annotate::format_acsl_annotations(input)
}

pub fn format_acsl_comment(input: &str) -> String {
    annotate::format_single_annotation(input)
}

pub fn format_acsl_annotation(content: &str) -> String {
    annotate::format_annotation_content(content, annotate::FormatStyle::Line)
}

pub fn format_expression(expr: &str) -> Result<String, Error> {
    let parsed = parser::parse_expression(expr)?;
    Ok(format::format_expr(&parsed))
}
