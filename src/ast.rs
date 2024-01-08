#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    Number,
    Float,
    Identifier,
    BinaryExpr,
    NullLiteral,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            kind: NodeType::Program,
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Number(Number),
    Float(Float),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),    
    NullLiteral(NullLiteral),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<Stmt>,
    pub right: Box<Stmt>,
    pub operator: char,
}

impl BinaryExpr {
    pub fn new(left: Box<Stmt>, right: Box<Stmt>, operator: char) -> Self {
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
    pub value: i64,
}

impl Number {
    pub fn new(value: i64) -> Self {
        Number {
            kind: NodeType::Number,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    pub kind: NodeType,
    pub value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Float {
            kind: NodeType::Float,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub kind: NodeType,
    pub value: String,
}

impl NullLiteral {
    pub fn new() -> Self {
        NullLiteral {
            kind: NodeType::NullLiteral,
            value: String::from("null"),
        }
    }
}
