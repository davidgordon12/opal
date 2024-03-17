use std::collections::HashMap;

use crate::ast::{BinaryExpr, LetDeclaration, Node, Number, OString};
use crate::runtime::values::Value;

pub struct OVM {
    ast: Vec<Node>,
    constants: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl OVM {
    pub fn new(ast: Vec<Node>) -> OVM {
        OVM {
            ast: ast,
            constants: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self) {
        let ast = self.ast.clone();

        for x in ast {
            match x {
                Node::LetDeclaration(x) => self.evaluate_let_decleration(x),
               _ => panic!(), 
            }
        }
    }

    fn add_constant(&mut self, key: String, value: Value) {
        match value {
            Value::OString(x) => { self.constants.insert(key, Value::OString(x)); },
            Value::Number(x) => { self.constants.insert(key, Value::Number(x)); },
            Value::Float(x) => { self.constants.insert(key, Value::Float(x)); },
        }
    }

    fn evaluate_let_decleration(&mut self, decleration: LetDeclaration) {
        let value = decleration.value;

        match *value {
            Node::OString(x) => self.add_constant(decleration.identifier, Value::OString(x.value)),
            Node::Number(x) => self.add_constant(decleration.identifier, Value::Number(x.value)),
            Node::Float(x) => self.add_constant(decleration.identifier, Value::Float(x.value)),
            Node::BinaryExpr(x) => self.add_constant(decleration.identifier, 
                self.evaluate_binary_expression(x)),
            
            _ => panic!()
        }
    }

    fn evaluate_binary_expression(&mut self, expr: BinaryExpr) -> Value {

    }
}