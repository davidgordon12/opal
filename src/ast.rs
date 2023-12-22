#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    Number,
    Identifier,
    BinaryExpr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),    
    ParseErr,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: NodeType,
    pub rigth: NodeType,
    pub operator: char,
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
    pub value: i32,
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