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