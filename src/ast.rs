#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(String),
    Number(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    Member {
        base: Box<Expr>,
        op: MemberOp,
        field: String,
    },
    Quant {
        kind: QuantKind,
        vars: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Implies,
    Iff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberOp {
    Dot,
    Arrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantKind {
    Forall,
    Exists,
}

impl UnaryOp {
    pub fn as_str(self) -> &'static str {
        match self {
            UnaryOp::Not => "!",
            UnaryOp::Neg => "-",
            UnaryOp::Pos => "+",
        }
    }
}

impl BinaryOp {
    pub fn as_str(self) -> &'static str {
        match self {
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Implies => "==>",
            BinaryOp::Iff => "<==>",
        }
    }

    pub fn precedence(self) -> u8 {
        match self {
            BinaryOp::Iff => 2,
            BinaryOp::Implies => 3,
            BinaryOp::Or => 4,
            BinaryOp::And => 5,
            BinaryOp::Eq | BinaryOp::Ne => 6,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 7,
            BinaryOp::Add | BinaryOp::Sub => 8,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 9,
        }
    }

    pub fn assoc(self) -> Assoc {
        match self {
            BinaryOp::Implies => Assoc::Right,
            _ => Assoc::Left,
        }
    }
}

impl QuantKind {
    pub fn as_str(self) -> &'static str {
        match self {
            QuantKind::Forall => "\\forall",
            QuantKind::Exists => "\\exists",
        }
    }
}
