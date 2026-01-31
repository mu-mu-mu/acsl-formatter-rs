use crate::format::format_expr;
use crate::parser::Parser;

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
    let had_trailing_semicolon = content.trim_end().ends_with(';');
    let mut parts = Vec::new();
    for part in content.split(';') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let formatted = match Parser::new(trimmed).and_then(|mut p| p.parse_expression()) {
            Ok(expr) => format_expr(&expr),
            Err(_) => trimmed.to_string(),
        };
        parts.push(formatted);
    }
    let mut joined = parts.join("; ");
    if had_trailing_semicolon {
        joined.push(';');
    }
    joined
}
