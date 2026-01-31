use crate::ast::{Assoc, Expr};

pub fn format_expr(expr: &Expr) -> String {
    format_expr_with_ctx(expr, 0, Assoc::Left, false)
}

fn format_expr_with_ctx(expr: &Expr, parent_prec: u8, parent_assoc: Assoc, is_right: bool) -> String {
    match expr {
        Expr::Ident(name) => name.clone(),
        Expr::Number(value) => value.clone(),
        Expr::Unary { op, expr } => {
            let inner = format_expr_with_ctx(expr, node_prec_unary(), Assoc::Right, false);
            let combined = format!("{}{}", op.as_str(), inner);
            if needs_parens(node_prec_unary(), parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
        Expr::Binary { op, left, right } => {
            let prec = op.precedence();
            let assoc = op.assoc();
            let left_str = format_expr_with_ctx(left, prec, assoc, false);
            let right_str = format_expr_with_ctx(right, prec, assoc, true);
            let combined = format!("{left_str} {} {right_str}", op.as_str());
            if needs_parens(prec, parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
    }
}

fn node_prec_unary() -> u8 {
    9
}

fn needs_parens(node_prec: u8, parent_prec: u8, parent_assoc: Assoc, is_right: bool) -> bool {
    if node_prec < parent_prec {
        return true;
    }
    if node_prec > parent_prec {
        return false;
    }
    match parent_assoc {
        Assoc::Left => is_right,
        Assoc::Right => !is_right,
    }
}
