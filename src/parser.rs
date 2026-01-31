use crate::ast::{BinaryOp, Expr, QuantKind, UnaryOp};
use crate::error::Error;
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
    lookahead: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(input);
        let lookahead = lexer.next_token()?;
        Ok(Self { lexer, lookahead })
    }

    pub fn parse_expression(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_expr_bp(0)?;
        if self.lookahead.kind != TokenKind::End {
            return Err(Error::Parse(format!(
                "unexpected token {:?} at {}",
                self.lookahead.kind, self.lookahead.pos
            )));
        }
        Ok(expr)
    }

    pub fn parse_annotation_list(&mut self) -> Result<(Vec<Expr>, bool), Error> {
        let mut exprs = Vec::new();
        if self.lookahead.kind == TokenKind::End {
            return Ok((exprs, false));
        }

        loop {
            let expr = self.parse_expr_bp(0)?;
            exprs.push(expr);

            match &self.lookahead.kind {
                TokenKind::Semicolon => {
                    self.bump()?;
                    if self.lookahead.kind == TokenKind::End {
                        return Ok((exprs, true));
                    }
                }
                TokenKind::End => return Ok((exprs, false)),
                kind => {
                    return Err(Error::Parse(format!(
                        "unexpected token {:?} at {}",
                        kind, self.lookahead.pos
                    )))
                }
            }
        }
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, Error> {
        let mut lhs = match &self.lookahead.kind {
            TokenKind::Ident(name) => {
                let value = name.clone();
                self.bump()?;
                Expr::Ident(value)
            }
            TokenKind::Number(value) => {
                let value = value.clone();
                self.bump()?;
                Expr::Number(value)
            }
            TokenKind::KwForall => {
                self.bump()?;
                self.parse_quant(QuantKind::Forall)?
            }
            TokenKind::KwExists => {
                self.bump()?;
                self.parse_quant(QuantKind::Exists)?
            }
            TokenKind::Op(op) if op == "!" || op == "-" || op == "+" => {
                let op = op.clone();
                self.bump()?;
                let (rbp, uop) = match op.as_str() {
                    "!" => (10, UnaryOp::Not),
                    "-" => (10, UnaryOp::Neg),
                    "+" => (10, UnaryOp::Pos),
                    _ => {
                        return Err(Error::Parse(format!(
                            "unsupported unary operator '{op}'"
                        )))
                    }
                };
                let expr = self.parse_expr_bp(rbp)?;
                Expr::Unary {
                    op: uop,
                    expr: Box::new(expr),
                }
            }
            TokenKind::LParen => {
                self.bump()?;
                let expr = self.parse_expr_bp(0)?;
                self.expect(TokenKind::RParen)?;
                expr
            }
            kind => {
                return Err(Error::Parse(format!(
                    "unexpected token {:?} at {}",
                    kind, self.lookahead.pos
                )))
            }
        };

        loop {
            match &self.lookahead.kind {
                TokenKind::LParen => {
                    self.bump()?;
                    let mut args = Vec::new();
                    if self.lookahead.kind != TokenKind::RParen {
                        loop {
                            let arg = self.parse_expr_bp(0)?;
                            args.push(arg);
                            if self.lookahead.kind == TokenKind::Comma {
                                self.bump()?;
                                continue;
                            }
                            break;
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    lhs = Expr::Call {
                        callee: Box::new(lhs),
                        args,
                    };
                }
                TokenKind::LBracket => {
                    self.bump()?;
                    let index = self.parse_expr_bp(0)?;
                    self.expect(TokenKind::RBracket)?;
                    lhs = Expr::Index {
                        base: Box::new(lhs),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        loop {
            if self.lookahead.kind == TokenKind::Question {
                let (lbp, rbp) = ternary_binding_power();
                if lbp < min_bp {
                    break;
                }
                self.bump()?;
                let then_expr = self.parse_expr_bp(rbp)?;
                self.expect(TokenKind::Colon)?;
                let else_expr = self.parse_expr_bp(rbp)?;
                lhs = Expr::Ternary {
                    cond: Box::new(lhs),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                };
                continue;
            }

            let op = match &self.lookahead.kind {
                TokenKind::Op(op) => op.clone(),
                _ => break,
            };

            let bop = match op.as_str() {
                "*" => BinaryOp::Mul,
                "/" => BinaryOp::Div,
                "%" => BinaryOp::Mod,
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                "<" => BinaryOp::Lt,
                "<=" => BinaryOp::Le,
                ">" => BinaryOp::Gt,
                ">=" => BinaryOp::Ge,
                "==" => BinaryOp::Eq,
                "!=" => BinaryOp::Ne,
                "&&" => BinaryOp::And,
                "||" => BinaryOp::Or,
                "==>" => BinaryOp::Implies,
                "<==>" => BinaryOp::Iff,
                _ => break,
            };

            let (lbp, rbp) = infix_binding_power(bop);
            if lbp < min_bp {
                break;
            }

            self.bump()?;
            let rhs = self.parse_expr_bp(rbp)?;
            lhs = Expr::Binary {
                op: bop,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn bump(&mut self) -> Result<(), Error> {
        self.lookahead = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), Error> {
        if self.lookahead.kind == expected {
            self.bump()?;
            Ok(())
        } else {
            Err(Error::Parse(format!(
                "expected {:?}, found {:?} at {}",
                expected, self.lookahead.kind, self.lookahead.pos
            )))
        }
    }

    fn parse_quant(&mut self, kind: QuantKind) -> Result<Expr, Error> {
        let mut vars = Vec::new();
        loop {
            match &self.lookahead.kind {
                TokenKind::Ident(name) => {
                    vars.push(name.clone());
                    self.bump()?;
                }
                _ => {
                    return Err(Error::Parse(format!(
                        "expected identifier after {}, found {:?} at {}",
                        kind.as_str(),
                        self.lookahead.kind,
                        self.lookahead.pos
                    )))
                }
            }
            if self.lookahead.kind == TokenKind::Comma {
                self.bump()?;
                continue;
            }
            break;
        }
        self.expect(TokenKind::Semicolon)?;
        let body = self.parse_expr_bp(0)?;
        Ok(Expr::Quant {
            kind,
            vars,
            body: Box::new(body),
        })
    }
}

fn infix_binding_power(op: BinaryOp) -> (u8, u8) {
    let prec = op.precedence();
    match op.assoc() {
        crate::ast::Assoc::Left => (prec, prec + 1),
        crate::ast::Assoc::Right => (prec, prec),
    }
}

fn ternary_binding_power() -> (u8, u8) {
    (1, 0)
}
