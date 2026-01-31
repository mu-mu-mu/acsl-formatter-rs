use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    Number(String),
    Op(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Question,
    Colon,
    KwForall,
    KwExists,
    End,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    idx: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_ws();
        let pos = self.idx;
        if self.idx >= self.chars.len() {
            return Ok(Token {
                kind: TokenKind::End,
                pos,
            });
        }

        let ch = self.chars[self.idx];
        if is_ident_start(ch) {
            let start = self.idx;
            self.idx += 1;
            while self.idx < self.chars.len() && is_ident_continue(self.chars[self.idx]) {
                self.idx += 1;
            }
            let text = self.slice(start, self.idx);
            let kind = match text.as_str() {
                "\\forall" => TokenKind::KwForall,
                "\\exists" => TokenKind::KwExists,
                _ => TokenKind::Ident(text),
            };
            return Ok(Token { kind, pos });
        }

        if ch.is_ascii_digit() {
            let start = self.idx;
            self.idx += 1;
            while self.idx < self.chars.len() && self.chars[self.idx].is_ascii_digit() {
                self.idx += 1;
            }
            let text = self.slice(start, self.idx);
            return Ok(Token {
                kind: TokenKind::Number(text),
                pos,
            });
        }

        match ch {
            '(' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::LParen,
                    pos,
                })
            }
            ')' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::RParen,
                    pos,
                })
            }
            '[' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::LBracket,
                    pos,
                })
            }
            ']' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::RBracket,
                    pos,
                })
            }
            ',' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::Comma,
                    pos,
                })
            }
            ';' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::Semicolon,
                    pos,
                })
            }
            '?' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::Question,
                    pos,
                })
            }
            ':' => {
                self.idx += 1;
                Ok(Token {
                    kind: TokenKind::Colon,
                    pos,
                })
            }
            _ => self.lex_operator_or_error(pos),
        }
    }

    fn lex_operator_or_error(&mut self, pos: usize) -> Result<Token, Error> {
        let ops = [
            "<==>", "==>", "&&", "||", "==", "!=", "<=", ">=", "<", ">", "+", "-", "*",
            "/", "%", "!",
        ];
        for op in ops {
            if self.match_str(op) {
                return Ok(Token {
                    kind: TokenKind::Op(op.to_string()),
                    pos,
                });
            }
        }
        Err(Error::Lex(format!(
            "unexpected character '{}' at {}",
            self.chars[self.idx],
            self.idx
        )))
    }

    fn match_str(&mut self, s: &str) -> bool {
        let end = self.idx + s.chars().count();
        if end > self.chars.len() {
            return false;
        }
        if self.slice(self.idx, end) == s {
            self.idx = end;
            return true;
        }
        false
    }

    fn slice(&self, start: usize, end: usize) -> String {
        self.chars[start..end].iter().collect()
    }

    fn skip_ws(&mut self) {
        while self.idx < self.chars.len() && self.chars[self.idx].is_whitespace() {
            self.idx += 1;
        }
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '\\'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '\\'
}
