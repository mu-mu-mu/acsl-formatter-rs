use crate::ast::{Assoc, BinaryOp, Expr, MemberOp, UnaryOp};

pub fn format_expr(expr: &Expr) -> String {
    let simplified = simplify_expr(expr);
    format_expr_with_ctx(&simplified, 0, Assoc::Left, false)
}

fn simplify_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Ident(_) | Expr::Number(_) => expr.clone(),
        Expr::Unary { op, expr } => {
            let inner = simplify_expr(expr);
            if *op != UnaryOp::Not {
                return Expr::Unary {
                    op: *op,
                    expr: Box::new(inner),
                };
            }
            match inner {
                Expr::Unary {
                    op: UnaryOp::Not,
                    expr,
                } => *expr,
                Expr::Binary {
                    op: bin_op,
                    left,
                    right,
                } => {
                    if let Some(inverted) = invert_comparison(bin_op) {
                        Expr::Binary {
                            op: inverted,
                            left,
                            right,
                        }
                    } else {
                        Expr::Unary {
                            op: *op,
                            expr: Box::new(Expr::Binary {
                                op: bin_op,
                                left,
                                right,
                            }),
                        }
                    }
                }
                other => Expr::Unary {
                    op: *op,
                    expr: Box::new(other),
                },
            }
        }
        Expr::Binary { op, left, right } => Expr::Binary {
            op: *op,
            left: Box::new(simplify_expr(left)),
            right: Box::new(simplify_expr(right)),
        },
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => Expr::Ternary {
            cond: Box::new(simplify_expr(cond)),
            then_expr: Box::new(simplify_expr(then_expr)),
            else_expr: Box::new(simplify_expr(else_expr)),
        },
        Expr::Call { callee, args } => Expr::Call {
            callee: Box::new(simplify_expr(callee)),
            args: args.iter().map(simplify_expr).collect(),
        },
        Expr::Index { base, index } => Expr::Index {
            base: Box::new(simplify_expr(base)),
            index: Box::new(simplify_expr(index)),
        },
        Expr::Member { base, op, field } => Expr::Member {
            base: Box::new(simplify_expr(base)),
            op: *op,
            field: field.clone(),
        },
        Expr::Quant { kind, vars, body } => Expr::Quant {
            kind: *kind,
            vars: vars.clone(),
            body: Box::new(simplify_expr(body)),
        },
    }
}

fn invert_comparison(op: BinaryOp) -> Option<BinaryOp> {
    match op {
        BinaryOp::Eq => Some(BinaryOp::Ne),
        BinaryOp::Ne => Some(BinaryOp::Eq),
        BinaryOp::Lt => Some(BinaryOp::Ge),
        BinaryOp::Le => Some(BinaryOp::Gt),
        BinaryOp::Gt => Some(BinaryOp::Le),
        BinaryOp::Ge => Some(BinaryOp::Lt),
        _ => None,
    }
}

fn format_expr_with_ctx(
    expr: &Expr,
    parent_prec: u8,
    parent_assoc: Assoc,
    is_right: bool,
) -> String {
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
            let left_str = format_binary_child(left, *op, prec, assoc, false);
            let right_str = format_binary_child(right, *op, prec, assoc, true);
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
        Expr::Member { base, op, field } => {
            let base_prec = prec_for_expr(base);
            let base_str = format_expr_with_ctx(base, PREC_POSTFIX, Assoc::Left, false);
            let base_str = if needs_parens(base_prec, PREC_POSTFIX, Assoc::Left, false) {
                format!("({base_str})")
            } else {
                base_str
            };
            let op_str = match op {
                MemberOp::Dot => ".",
                MemberOp::Arrow => "->",
            };
            let combined = format!("{base_str}{op_str}{field}");
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

fn format_binary_child(
    expr: &Expr,
    parent_op: BinaryOp,
    parent_prec: u8,
    parent_assoc: Assoc,
    is_right: bool,
) -> String {
    let formatted = format_expr_with_ctx(expr, parent_prec, parent_assoc, is_right);
    if needs_binary_child_parens(expr, parent_op) {
        format!("({formatted})")
    } else {
        formatted
    }
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

fn needs_binary_child_parens(expr: &Expr, parent_op: BinaryOp) -> bool {
    matches!(
        (parent_op, expr),
        (
            BinaryOp::Or,
            Expr::Binary {
                op: BinaryOp::And,
                ..
            }
        ) | (
            BinaryOp::Eq | BinaryOp::Ne,
            Expr::Binary {
                op: BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge,
                ..
            }
        )
    )
}

fn prec_for_expr(expr: &Expr) -> u8 {
    match expr {
        Expr::Quant { .. } => PREC_QUANT,
        Expr::Ternary { .. } => PREC_TERNARY,
        Expr::Binary { op, .. } => op.precedence(),
        Expr::Unary { .. } => PREC_UNARY,
        Expr::Call { .. } | Expr::Index { .. } | Expr::Member { .. } => PREC_POSTFIX,
        Expr::Ident(_) | Expr::Number(_) => PREC_PRIMARY,
    }
}
