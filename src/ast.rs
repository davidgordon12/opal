use crate::error::error;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    Number,
    Identifier,
    BinaryExpr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),    
}

impl Expr {
    pub fn unwrap_binary_expr(self) -> BinaryExpr {
        match self {
            Expr::BinaryExpr(x) => return x,
            _ => error("Expeted binary expression.", None)
        }

        unreachable!()
    }

    pub fn unwrap_identifier(self) -> Identifier {
        match self {
            Expr::Identifier(x) => return x,
            _ => error("Expeted identifier.", None)
        }

        unreachable!()
    }

    pub fn unwrap_number(self) -> Number {
        match self {
            Expr::Number(x) => return x,
            _ => error("Expeted number.", None),
        }

        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: String,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, right: Box<Expr>, operator: String) -> Self {
        BinaryExpr {
            kind: NodeType::BinaryExpr,
            left: left,
            right: right,
            operator: operator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String
}

impl Identifier {
    pub fn new(symbol: String) -> Self {
        Identifier {
            kind: NodeType::Identifier,
            symbol: symbol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub kind: NodeType,
    pub value: f32,
}

impl Number {
    pub fn new(value: f32) -> Self {
        Number {
            kind: NodeType::Number,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Expr>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            kind: NodeType::Program,
            body: Vec::new(),
        }
    }
}