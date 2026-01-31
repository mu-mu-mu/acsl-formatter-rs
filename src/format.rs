use crate::ast::{Assoc, Expr};

pub fn format_expr(expr: &Expr) -> String {
    format_expr_with_ctx(expr, 0, Assoc::Left, false)
}

fn format_expr_with_ctx(expr: &Expr, parent_prec: u8, parent_assoc: Assoc, is_right: bool) -> String {
    match expr {
        Expr::Ident(name) => name.clone(),
        Expr::Number(value) => value.clone(),
        Expr::Unary { op, expr } => {
            let inner = format_expr_with_ctx(expr, PREC_UNARY, Assoc::Right, false);
            let combined = format!("{}{}", op.as_str(), inner);
            if needs_parens(PREC_UNARY, parent_prec, parent_assoc, is_right) {
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
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => {
            let prec = PREC_TERNARY;
            let cond_str = format_expr_with_ctx(cond, prec, Assoc::Right, false);
            let then_str = format_expr_with_ctx(then_expr, prec, Assoc::Right, false);
            let else_str = format_expr_with_ctx(else_expr, prec, Assoc::Right, true);
            let combined = format!("{cond_str} ? {then_str} : {else_str}");
            if needs_parens(prec, parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
        Expr::Call { callee, args } => {
            let callee_prec = prec_for_expr(callee);
            let callee_str = format_expr_with_ctx(callee, PREC_POSTFIX, Assoc::Left, false);
            let callee_str = if needs_parens(callee_prec, PREC_POSTFIX, Assoc::Left, false) {
                format!("({callee_str})")
            } else {
                callee_str
            };
            let args_str = args
                .iter()
                .map(|arg| format_expr_with_ctx(arg, 0, Assoc::Left, false))
                .collect::<Vec<_>>()
                .join(", ");
            let combined = format!("{callee_str}({args_str})");
            if needs_parens(PREC_POSTFIX, parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
        Expr::Index { base, index } => {
            let base_prec = prec_for_expr(base);
            let base_str = format_expr_with_ctx(base, PREC_POSTFIX, Assoc::Left, false);
            let base_str = if needs_parens(base_prec, PREC_POSTFIX, Assoc::Left, false) {
                format!("({base_str})")
            } else {
                base_str
            };
            let index_str = format_expr_with_ctx(index, 0, Assoc::Left, false);
            let combined = format!("{base_str}[{index_str}]");
            if needs_parens(PREC_POSTFIX, parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
        Expr::Quant { kind, vars, body } => {
            let prec = PREC_QUANT;
            let vars_str = vars.join(", ");
            let body_str = format_expr_with_ctx(body, prec, Assoc::Right, true);
            let combined = format!("{} {vars_str}; {body_str}", kind.as_str());
            if needs_parens(prec, parent_prec, parent_assoc, is_right) {
                format!("({combined})")
            } else {
                combined
            }
        }
    }
}

const PREC_QUANT: u8 = 0;
const PREC_TERNARY: u8 = 1;
const PREC_UNARY: u8 = 10;
const PREC_POSTFIX: u8 = 11;
const PREC_PRIMARY: u8 = 12;

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

fn prec_for_expr(expr: &Expr) -> u8 {
    match expr {
        Expr::Quant { .. } => PREC_QUANT,
        Expr::Ternary { .. } => PREC_TERNARY,
        Expr::Binary { op, .. } => op.precedence(),
        Expr::Unary { .. } => PREC_UNARY,
        Expr::Call { .. } | Expr::Index { .. } => PREC_POSTFIX,
        Expr::Ident(_) | Expr::Number(_) => PREC_PRIMARY,
    }
}
