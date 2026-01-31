use crate::format::format_expr;
use crate::parser;

pub fn format_acsl_annotations(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '/' && i + 2 < chars.len() && chars[i + 1] == '*' && chars[i + 2] == '@' {
            out.push_str("/*@");
            i += 3;
            let mut content = String::new();
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                content.push(chars[i]);
                i += 1;
            }
            if i + 1 < chars.len() {
                let formatted = format_annotation_content(&content);
                if !formatted.is_empty() {
                    out.push(' ');
                    out.push_str(&formatted);
                    out.push(' ');
                }
                out.push_str("*/");
                i += 2;
            } else {
                out.push_str(&content);
                break;
            }
        } else if chars[i] == '/' && i + 2 < chars.len() && chars[i + 1] == '/' && chars[i + 2] == '@' {
            out.push_str("//@");
            i += 3;
            let mut content = String::new();
            while i < chars.len() && chars[i] != '\n' {
                content.push(chars[i]);
                i += 1;
            }
            let formatted = format_annotation_content(&content);
            if !formatted.is_empty() {
                out.push(' ');
                out.push_str(&formatted);
            }
            if i < chars.len() && chars[i] == '\n' {
                out.push('\n');
                i += 1;
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

pub fn format_annotation_content(content: &str) -> String {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    match parser::parse_annotation_list(trimmed) {
        Ok((exprs, trailing)) => {
            let mut joined = exprs
                .iter()
                .map(format_expr)
                .collect::<Vec<_>>()
                .join("; ");
            if trailing {
                joined.push(';');
            }
            joined
        }
        Err(_) => trimmed.to_string(),
    }
}
