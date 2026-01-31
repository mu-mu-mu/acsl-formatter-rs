use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::error::Error;
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, Error> {
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
            TokenKind::Op(op) if op == "!" || op == "-" || op == "+" => {
                let op = op.clone();
                self.bump()?;
                let (rbp, uop) = match op.as_str() {
                    "!" => (9, UnaryOp::Not),
                    "-" => (9, UnaryOp::Neg),
                    "+" => (9, UnaryOp::Pos),
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
}

fn infix_binding_power(op: BinaryOp) -> (u8, u8) {
    let prec = op.precedence();
    match op.assoc() {
        crate::ast::Assoc::Left => (prec, prec + 1),
        crate::ast::Assoc::Right => (prec, prec),
    }
}
