#[derive(Debug, Clone)]
pub enum Value {
    OString(String),
    Number(i64),
    Float(f64),
}