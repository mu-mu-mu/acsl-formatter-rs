use crate::ast::{BinaryOp, Expr, QuantKind, UnaryOp};
use crate::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct AcslParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClauseKind {
    Expr,
    Assert,
    LoopInvariant,
    Requires,
    Ensures,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub kind: ClauseKind,
    pub expr: Expr,
}

pub fn parse_expression(input: &str) -> Result<Expr, Error> {
    let mut pairs = AcslParser::parse(Rule::expr_input, input)
        .map_err(|e| Error::Parse(e.to_string()))?;
    let pair = pairs
        .next()
        .ok_or_else(|| Error::Parse("missing expression".to_string()))?;
    let expr_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| Error::Parse("missing expression".to_string()))?;
    parse_expr(expr_pair)
}

pub fn parse_annotation_list(input: &str) -> Result<(Vec<Clause>, bool), Error> {
    let mut pairs = AcslParser::parse(Rule::expr_list, input)
        .map_err(|e| Error::Parse(e.to_string()))?;
    let pair = pairs
        .next()
        .ok_or_else(|| Error::Parse("missing expr_list".to_string()))?;
    let mut clauses = Vec::new();
    for next in pair.into_inner() {
        if next.as_rule() == Rule::clause {
            clauses.push(parse_clause(next)?);
        }
    }
    let trailing = input.trim_end().ends_with(';');
    Ok((clauses, trailing))
}

fn parse_clause(pair: Pair<Rule>) -> Result<Clause, Error> {
    let mut inner = pair.into_inner();
    let first = inner
        .next()
        .ok_or_else(|| Error::Parse("missing clause".to_string()))?;
    match first.as_rule() {
        Rule::loop_invariant => {
            let mut inner = first.into_inner();
            let _loop_kw = inner.next();
            let _inv_kw = inner.next();
            let expr_pair = inner
                .next()
                .ok_or_else(|| Error::Parse("missing loop invariant expr".to_string()))?;
            let expr = parse_expr(expr_pair)?;
            Ok(Clause {
                kind: ClauseKind::LoopInvariant,
                expr,
            })
        }
        Rule::assert_clause => {
            let mut inner = first.into_inner();
            let _assert_kw = inner.next();
            let expr_pair = inner
                .next()
                .ok_or_else(|| Error::Parse("missing assert expr".to_string()))?;
            let expr = parse_expr(expr_pair)?;
            Ok(Clause {
                kind: ClauseKind::Assert,
                expr,
            })
        }
        Rule::requires_clause => {
            let mut inner = first.into_inner();
            let _kw = inner.next();
            let expr_pair = inner
                .next()
                .ok_or_else(|| Error::Parse("missing requires expr".to_string()))?;
            let expr = parse_expr(expr_pair)?;
            Ok(Clause {
                kind: ClauseKind::Requires,
                expr,
            })
        }
        Rule::ensures_clause => {
            let mut inner = first.into_inner();
            let _kw = inner.next();
            let expr_pair = inner
                .next()
                .ok_or_else(|| Error::Parse("missing ensures expr".to_string()))?;
            let expr = parse_expr(expr_pair)?;
            Ok(Clause {
                kind: ClauseKind::Ensures,
                expr,
            })
        }
        Rule::expr => {
            let expr = parse_expr(first)?;
            Ok(Clause {
                kind: ClauseKind::Expr,
                expr,
            })
        }
        _ => Err(Error::Parse("unexpected clause".to_string())),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::ternary => parse_ternary(pair),
        Rule::iff_expr => parse_left_assoc(pair, parse_implies_expr),
        Rule::implies_expr => parse_right_assoc(pair, parse_or_expr),
        Rule::or_expr => parse_left_assoc(pair, parse_and_expr),
        Rule::and_expr => parse_left_assoc(pair, parse_eq_expr),
        Rule::eq_expr => parse_left_assoc(pair, parse_rel_expr),
        Rule::rel_expr => parse_left_assoc(pair, parse_add_expr),
        Rule::add_expr => parse_left_assoc(pair, parse_mul_expr),
        Rule::mul_expr => parse_left_assoc(pair, parse_unary_expr),
        Rule::unary_expr => parse_unary_expr(pair),
        Rule::postfix_expr => parse_postfix(pair),
        Rule::primary => parse_primary(pair),
        Rule::quant => parse_quant(pair),
        Rule::number => Ok(Expr::Number(pair.as_str().to_string())),
        Rule::ident => Ok(Expr::Ident(pair.as_str().to_string())),
        _ => Err(Error::Parse(format!("unexpected rule {:?}", pair.as_rule()))),
    }
}

fn parse_ternary(pair: Pair<Rule>) -> Result<Expr, Error> {
    let mut inner = pair.into_inner();
    let cond_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing ternary condition".to_string()))?;
    let cond = parse_expr(cond_pair)?;
    let then_pair = inner.next();
    if then_pair.is_none() {
        return Ok(cond);
    }
    let then_expr = parse_expr(then_pair.unwrap())?;
    let else_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing ternary else".to_string()))?;
    let else_expr = parse_expr(else_pair)?;
    Ok(Expr::Ternary {
        cond: Box::new(cond),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    })
}

fn parse_implies_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_right_assoc(pair, parse_or_expr)
}

fn parse_or_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_and_expr)
}

fn parse_and_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_eq_expr)
}

fn parse_eq_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_rel_expr)
}

fn parse_rel_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_add_expr)
}

fn parse_add_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_mul_expr)
}

fn parse_mul_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    parse_left_assoc(pair, parse_unary_expr)
}

fn parse_unary_expr(pair: Pair<Rule>) -> Result<Expr, Error> {
    let mut inner = pair.into_inner().peekable();
    let mut ops = Vec::new();
    while let Some(next) = inner.peek() {
        if next.as_rule() == Rule::op_unary {
            ops.push(next.as_str().to_string());
            inner.next();
        } else {
            break;
        }
    }
    let expr_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing unary operand".to_string()))?;
    let mut expr = parse_postfix(expr_pair)?;
    for op in ops.into_iter().rev() {
        let uop = match op.as_str() {
            "!" => UnaryOp::Not,
            "+" => UnaryOp::Pos,
            "-" => UnaryOp::Neg,
            _ => return Err(Error::Parse(format!("unknown unary op {op}"))),
        };
        expr = Expr::Unary {
            op: uop,
            expr: Box::new(expr),
        };
    }
    Ok(expr)
}

fn parse_postfix(pair: Pair<Rule>) -> Result<Expr, Error> {
    let mut inner = pair.into_inner();
    let mut expr = parse_primary(inner.next().ok_or_else(|| Error::Parse("missing primary".to_string()))?)?;
    for postfix in inner {
        match postfix.as_rule() {
            Rule::call => {
                let mut args = Vec::new();
                for arg in postfix.into_inner() {
                    if arg.as_rule() == Rule::expr {
                        args.push(parse_expr(arg)?);
                    }
                }
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            }
            Rule::index => {
                let mut inner = postfix.into_inner();
                let idx_pair = inner
                    .next()
                    .ok_or_else(|| Error::Parse("missing index".to_string()))?;
                let index = parse_expr(idx_pair)?;
                expr = Expr::Index {
                    base: Box::new(expr),
                    index: Box::new(index),
                };
            }
            _ => {}
        }
    }
    Ok(expr)
}

fn parse_primary(pair: Pair<Rule>) -> Result<Expr, Error> {
    let mut inner = pair.into_inner();
    let node = inner
        .next()
        .ok_or_else(|| Error::Parse("missing primary".to_string()))?;
    match node.as_rule() {
        Rule::quant => parse_quant(node),
        Rule::number => Ok(Expr::Number(node.as_str().to_string())),
        Rule::ident => Ok(Expr::Ident(node.as_str().to_string())),
        Rule::expr => parse_expr(node),
        _ => Err(Error::Parse(format!("unexpected primary {:?}", node.as_rule()))),
    }
}

fn parse_quant(pair: Pair<Rule>) -> Result<Expr, Error> {
    let mut inner = pair.into_inner();
    let kind_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing quantifier".to_string()))?;
    let kind = match kind_pair.as_rule() {
        Rule::forall => QuantKind::Forall,
        Rule::exists => QuantKind::Exists,
        _ => return Err(Error::Parse("invalid quantifier".to_string())),
    };
    let vars_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing quantifier vars".to_string()))?;
    let vars = vars_pair
        .into_inner()
        .filter(|p| p.as_rule() == Rule::ident)
        .map(|p| p.as_str().to_string())
        .collect::<Vec<_>>();
    let body_pair = inner
        .next()
        .ok_or_else(|| Error::Parse("missing quantifier body".to_string()))?;
    let body = parse_expr(body_pair)?;
    Ok(Expr::Quant {
        kind,
        vars,
        body: Box::new(body),
    })
}

fn parse_left_assoc<F>(pair: Pair<Rule>, parse_next: F) -> Result<Expr, Error>
where
    F: Fn(Pair<Rule>) -> Result<Expr, Error>,
{
    let mut inner = pair.into_inner();
    let mut expr = parse_next(inner.next().ok_or_else(|| Error::Parse("missing lhs".to_string()))?)?;
    while let Some(op_pair) = inner.next() {
        let rhs_pair = inner
            .next()
            .ok_or_else(|| Error::Parse("missing rhs".to_string()))?;
        let op = map_bin_op_rule(op_pair.as_rule())
            .ok_or_else(|| Error::Parse(format!("unknown operator {:?}", op_pair.as_rule())))?;
        let rhs = parse_next(rhs_pair)?;
        expr = Expr::Binary {
            op,
            left: Box::new(expr),
            right: Box::new(rhs),
        };
    }
    Ok(expr)
}

fn parse_right_assoc<F>(pair: Pair<Rule>, parse_next: F) -> Result<Expr, Error>
where
    F: Fn(Pair<Rule>) -> Result<Expr, Error>,
{
    let mut inner = pair.into_inner();
    let mut exprs = Vec::new();
    let mut ops = Vec::new();
    if let Some(first) = inner.next() {
        exprs.push(parse_next(first)?);
    }
    while let Some(op_pair) = inner.next() {
        let rhs_pair = inner
            .next()
            .ok_or_else(|| Error::Parse("missing rhs".to_string()))?;
        ops.push(op_pair.as_rule());
        exprs.push(parse_next(rhs_pair)?);
    }
    if exprs.is_empty() {
        return Err(Error::Parse("missing rhs".to_string()));
    }
    let mut expr = exprs.pop().unwrap();
    while let Some(op_rule) = ops.pop() {
        let lhs = exprs
            .pop()
            .ok_or_else(|| Error::Parse("missing lhs".to_string()))?;
        let op = map_bin_op_rule(op_rule)
            .ok_or_else(|| Error::Parse(format!("unknown operator {:?}", op_rule)))?;
        expr = Expr::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(expr),
        };
    }
    Ok(expr)
}

fn map_bin_op_rule(rule: Rule) -> Option<BinaryOp> {
    match rule {
        Rule::op_mul => Some(BinaryOp::Mul),
        Rule::op_div => Some(BinaryOp::Div),
        Rule::op_mod => Some(BinaryOp::Mod),
        Rule::op_add => Some(BinaryOp::Add),
        Rule::op_sub => Some(BinaryOp::Sub),
        Rule::op_lt => Some(BinaryOp::Lt),
        Rule::op_le => Some(BinaryOp::Le),
        Rule::op_gt => Some(BinaryOp::Gt),
        Rule::op_ge => Some(BinaryOp::Ge),
        Rule::op_eq => Some(BinaryOp::Eq),
        Rule::op_ne => Some(BinaryOp::Ne),
        Rule::op_and => Some(BinaryOp::And),
        Rule::op_or => Some(BinaryOp::Or),
        Rule::op_implies => Some(BinaryOp::Implies),
        Rule::op_iff => Some(BinaryOp::Iff),
        _ => None,
    }
}
