use crate::tokens::TokenType;

#[derive(Debug, Clone)]
pub enum Node {
    Number(Number),
    Float(Float),
    OString(OString),
    Boolean(Boolean),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
    LetDeclaration(LetDeclaration),
    ProcDeclaration(ProcDeclaration),
    ReturnStatement(ReturnStatement),
    PrintStatement(PrintStatement),
    IfStatement(IfStatement),
    ProcedureCall(ProcedureCall),
}

#[derive(Debug, Clone)]
pub struct LetDeclaration {
    pub identifier: String,
    pub value: Box<Node>,
}

impl LetDeclaration {
    pub fn new(identifier: String, value: Box<Node>) -> Self {
        LetDeclaration {
            identifier: identifier,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcDeclaration {
    pub identifier: String,
    pub ret_value: TokenType,
    pub body: Vec<Node>,
}

impl ProcDeclaration {
    pub fn new(identifier: String, ret_value: TokenType) -> Self {
        ProcDeclaration {
            identifier: identifier,
            ret_value: ret_value,
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub operator: String,
}

impl BinaryExpr {
    pub fn new(left: Box<Node>, right: Box<Node>, operator: String) -> Self {
        BinaryExpr {
            left: left,
            right: right,
            operator: operator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub symbol: String,
}

impl Identifier {
    pub fn new(symbol: String) -> Self {
        Identifier { symbol: symbol }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i64,
}

impl Number {
    pub fn new(value: i64) -> Self {
        Number { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    pub value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Float { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct OString {
    pub value: String,
}

impl OString {
    pub fn new(value: String) -> Self {
        OString { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Box<Node>>,
}

impl ReturnStatement {
    pub fn new(value: Option<Box<Node>>) -> Self {
        ReturnStatement { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub value: Box<Node>,
}

impl PrintStatement {
    pub fn new(value: Box<Node>) -> Self {
        PrintStatement { value: value }
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub expr: Box<Node>,
    pub body: Vec<Node>,
}

impl IfStatement {
    pub fn new(expr: Box<Node>) -> Self {
        IfStatement {
            expr: expr,
            body: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcedureCall {
    pub args: Vec<Identifier>,
    pub caller: Identifier,
}

impl ProcedureCall {
    pub fn new(args: Vec<Identifier>, caller: Identifier) -> Self {
        ProcedureCall {
            args: args,
            caller: caller,
        }
    }
}
