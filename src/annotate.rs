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
        } else if chars[i] == '/'
            && i + 2 < chars.len()
            && chars[i + 1] == '/'
            && chars[i + 2] == '@'
        {
            out.push_str("//@");
            i += 3;
            let mut content = String::new();
            while i < chars.len() && chars[i] != '\n' {
                content.push(chars[i]);
                i += 1;
            }
            let formatted = format_annotation_content(&content, FormatStyle::Line);
            if !formatted.is_empty() {
                let mut first = true;
                for line in formatted.split('\n') {
                    if !first {
                        out.push('\n');
                        out.push_str("//@ ");
                    }
                    if first {
                        out.push(' ');
                        first = false;
                    }
                    out.push_str(line);
                }
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

pub fn format_single_annotation(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("/*@") && trimmed.ends_with("*/") {
        return format_acsl_annotations(trimmed);
    }
    if trimmed.starts_with("//@") && !trimmed.contains('\n') {
        return format_acsl_annotations(trimmed);
    }
    input.to_string()
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
                    ClauseKind::Expr(expr) => normalize_redundant_spaces(&format_expr(expr)),
                    ClauseKind::Assert(expr) => {
                        format!("assert {}", normalize_redundant_spaces(&format_expr(expr)))
                    }
                    ClauseKind::LoopInvariant(expr) => {
                        format!(
                            "loop invariant {}",
                            normalize_redundant_spaces(&format_expr(expr))
                        )
                    }
                    ClauseKind::Requires(expr) => {
                        format!(
                            "requires {}",
                            normalize_redundant_spaces(&format_expr(expr))
                        )
                    }
                    ClauseKind::Ensures(expr) => {
                        format!("ensures {}", normalize_redundant_spaces(&format_expr(expr)))
                    }
                    ClauseKind::Assumes(expr) => {
                        format!("assumes {}", normalize_redundant_spaces(&format_expr(expr)))
                    }
                    ClauseKind::Assigns(items) => {
                        let list = items
                            .iter()
                            .map(|expr| normalize_redundant_spaces(&format_expr(expr)))
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
                FormatStyle::Block => format_block_lines(parts),
                FormatStyle::Line => format_line_text(parts),
            }
        }
        Err(_) => trimmed.to_string(),
    }
}

fn format_block_lines(parts: Vec<String>) -> String {
    let mut lines = Vec::new();
    let mut in_behavior = false;
    for part in parts {
        let is_behavior = part.starts_with("behavior ");
        if is_behavior {
            lines.push(format!("  {part}"));
            in_behavior = true;
            continue;
        }
        let hang = hanging_indent(&part);
        let mut wrapped = wrap_with_hang(&part, 80 - 2, hang);
        if let Some(last) = wrapped.last_mut() {
            last.push(';');
        }
        for (idx, line) in wrapped.into_iter().enumerate() {
            if idx == 0 {
                if in_behavior {
                    lines.push(format!("    {line}"));
                } else {
                    lines.push(format!("  {line}"));
                }
            } else {
                if in_behavior {
                    lines.push(format!("    {}{line}", " ".repeat(hang)));
                } else {
                    lines.push(format!("  {}{line}", " ".repeat(hang)));
                }
            }
        }
    }
    lines.join("\n")
}

fn format_line_text(parts: Vec<String>) -> String {
    let joined = parts.join("; ");
    let wrapped = wrap_line(&joined, 80 - 4);
    wrapped.join("\n")
}

fn wrap_line(line: &str, max_len: usize) -> Vec<String> {
    let trimmed = line.trim();
    if trimmed.len() <= max_len {
        return vec![trimmed.to_string()];
    }
    let mut out = Vec::new();
    let mut current = String::new();
    for word in trimmed.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
            continue;
        }
        let next_len = current.len() + 1 + word.len();
        if next_len > max_len {
            out.push(current);
            current = word.to_string();
        } else {
            current.push(' ');
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn wrap_with_hang(line: &str, max_len: usize, hang: usize) -> Vec<String> {
    let trimmed = line.trim();
    if trimmed.len() <= max_len {
        return vec![trimmed.to_string()];
    }
    let mut out = Vec::new();
    let mut current = String::new();
    let mut limit = max_len;
    for word in trimmed.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
            continue;
        }
        let next_len = current.len() + 1 + word.len();
        if next_len > limit {
            out.push(current);
            current = word.to_string();
            limit = max_len.saturating_sub(hang);
        } else {
            current.push(' ');
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

fn hanging_indent(line: &str) -> usize {
    let keywords = [
        "requires ",
        "ensures ",
        "assumes ",
        "assigns ",
        "assert ",
        "loop invariant ",
    ];
    for kw in keywords {
        if line.starts_with(kw) {
            if kw == "ensures " {
                if let Some(idx) = line.find("== ") {
                    return idx + 3;
                }
            }
            return kw.len();
        }
    }
    0
}

fn normalize_redundant_spaces(input: &str) -> String {
    let mut s = input.to_string();
    let replacements = [
        (" -> ", "->"),
        ("-> ", "->"),
        (" ->", "->"),
        (" . ", "."),
        (". ", "."),
        (" .", "."),
        (" [", "["),
        ("[ ", "["),
        (" ]", "]"),
    ];
    for (from, to) in replacements {
        while s.contains(from) {
            s = s.replace(from, to);
        }
    }
    s
}
