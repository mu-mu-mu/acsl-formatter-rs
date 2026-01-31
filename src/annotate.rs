use crate::format::format_expr;
use crate::parser::{self, ClauseKind};

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
                let formatted = format_annotation_content(&content, FormatStyle::Block);
                if !formatted.is_empty() {
                    out.push('\n');
                    out.push_str(&formatted);
                    out.push('\n');
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
            let formatted = format_annotation_content(&content, FormatStyle::Line);
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

pub enum FormatStyle {
    Block,
    Line,
}

pub fn format_annotation_content(content: &str, style: FormatStyle) -> String {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    match parser::parse_annotation_list(trimmed) {
        Ok((clauses, _trailing)) => {
            let parts = clauses
                .iter()
                .map(|clause| match clause {
                    ClauseKind::Expr(expr) => format_expr(expr),
                    ClauseKind::Assert(expr) => format!("assert {}", format_expr(expr)),
                    ClauseKind::LoopInvariant(expr) => {
                        format!("loop invariant {}", format_expr(expr))
                    }
                    ClauseKind::Requires(expr) => format!("requires {}", format_expr(expr)),
                    ClauseKind::Ensures(expr) => format!("ensures {}", format_expr(expr)),
                    ClauseKind::Assumes(expr) => format!("assumes {}", format_expr(expr)),
                    ClauseKind::Assigns(items) => {
                        let list = items
                            .iter()
                            .map(format_expr)
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("assigns {}", list)
                    }
                    ClauseKind::Behavior(name) => format!("behavior {name}:"),
                })
                .collect::<Vec<_>>();
            if parts.is_empty() {
                return String::new();
            }
            match style {
                FormatStyle::Block => {
                    let mut lines = Vec::new();
                    for part in parts {
                        let needs_semicolon = !part.starts_with("behavior ");
                        if needs_semicolon {
                            lines.push(format!("  {part};"));
                        } else {
                            lines.push(format!("  {part}"));
                        }
                    }
                    lines.join("\n")
                }
                FormatStyle::Line => parts.join("; "),
            }
        }
        Err(_) => trimmed.to_string(),
    }
}
