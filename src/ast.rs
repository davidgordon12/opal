#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    Number,
    Identifier,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    FunctionDeclaration
}