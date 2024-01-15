#[derive(Debug, Clone)]
pub enum Stmt {
    Number(Number),
    Float(Float),
    OString(OString),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),    
    NullLiteral(NullLiteral),
    LetDeclaration(LetDeclaration),
    ProcDeclaration(ProcDeclaration),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetDeclaration {
    pub identifier: String,
    pub value: Box<Stmt>,
}

impl LetDeclaration {
    pub fn new(identifier: String, value: Box<Stmt>) -> Self {
        LetDeclaration {
            identifier: identifier,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcDeclaration {
    pub identifier: String,
    pub body: Vec<Stmt>,
}

impl ProcDeclaration {
    pub fn new(identifier: String) -> Self {
        ProcDeclaration {
            identifier: identifier,
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Stmt>,
    pub right: Box<Stmt>,
    pub operator: char,
}

impl BinaryExpr {
    pub fn new(left: Box<Stmt>, right: Box<Stmt>, operator: char) -> Self {
        BinaryExpr {
            left: left,
            right: right,
            operator: operator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub symbol: String
}

impl Identifier {
    pub fn new(symbol: String) -> Self {
        Identifier {
            symbol: symbol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i64,
}

impl Number {
    pub fn new(value: i64) -> Self {
        Number {
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    pub value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Float {
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OString {
    pub value: String,
}

impl OString {
    pub fn new(value: String) -> Self {
        OString {
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub value: String,
}

impl NullLiteral {
    pub fn new() -> Self {
        NullLiteral {
            value: String::from("null"),
        }
    }
}
