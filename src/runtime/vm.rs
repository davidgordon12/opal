use std::collections::HashMap;

use crate::ast::{BinaryExpr, LetDeclaration, Node, PrintStatement};
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
                Node::PrintStatement(x) => self.evaluate_print_statement(x),
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

    fn get_constant(&mut self, key: String) -> Value {
        self.constants.get(&key).unwrap().clone()
    }

    fn evaluate_let_decleration(&mut self, decleration: LetDeclaration) {
        let value = decleration.value;

        match *value {
            Node::OString(x) => self.add_constant(decleration.identifier, Value::OString(x.value)),
            Node::Number(x) => self.add_constant(decleration.identifier, Value::Number(x.value)),
            Node::Float(x) => self.add_constant(decleration.identifier, Value::Float(x.value)),
            Node::BinaryExpr(x) => {
                let val = self.evaluate_binary_expression(x);
                self.add_constant(decleration.identifier, val);
            },
            _ => panic!()
        }
    }

    fn evaluate_print_statement(&mut self, stmt: PrintStatement) {
        let value = *stmt.value;

        match value {
            Node::Number(x) => println!("{}", x.value),
            Node::Float(x) => println!("{}", x.value),
            Node::OString(x) => println!("{}", x.value),
            Node::BinaryExpr(x) => {
                let value = self.evaluate_binary_expression(x);
                match value {
                    Value::OString(x) => println!("{}", x),
                    Value::Number(x) => println!("{}", x),
                    Value::Float(x) => println!("{}", x),
                }
            }
            Node::Identifier(x) => {
                let value = self.get_constant(x.symbol);
                match value {
                    Value::OString(x) => println!("{}", x),
                    Value::Number(x) => println!("{}", x),
                    Value::Float(x) => println!("{}", x),
                }
            }
            _ => panic!(),
        }
    }

    fn evaluate_binary_expression(&mut self, expr: BinaryExpr) -> Value {
        let lhs = *expr.left;

        match lhs {
            Node::Number(x) => { self.stack.push(Value::Number(x.value)); },
            Node::Float(x) => { self.stack.push(Value::Float(x.value)); },
            Node::OString(x) => { self.stack.push(Value::OString(x.value)); },
            Node::BinaryExpr(x) => { 
                let val = self.evaluate_binary_expression(x);
                self.stack.push(val)
            },
            _ => panic!(),
        }

        let rhs = *expr.right;

        match rhs {
            Node::Number(x) =>  { self.stack.push(Value::Number(x.value)); },
            Node::Float(x) => { self.stack.push(Value::Float(x.value)); },
            Node::OString(x) => { self.stack.push(Value::OString(x.value)); },
            Node::BinaryExpr(x) => { 
                let val = self.evaluate_binary_expression(x);
                self.stack.push(val)
            },
            _ => panic!(),
        }

        /* Horrifying code, but a brighter way to solve this hasn't come to me yet */
        let operator: char = expr.operator;

        let right_val = self.stack.pop().unwrap();
        let left_val = self.stack.pop().unwrap();

        match left_val {
            Value::Number(l) => {
                match right_val {
                    Value::Number(r) => {
                        match operator {
                            '+' => { return Value::Number(l + r) },
                            '-' => { return Value::Number(l - r) },
                            '*' => { return Value::Number(l * r) },
                            '/' => { return Value::Float(l as f64 / r as f64) },
                            '%' => { return Value::Float(l as f64 % r as f64) },
                            '^' => { return Value::Float((l as f64).powf(r as f64)) },
                            _ => panic!(),
                        }
                    },
                    Value::Float(r) => {
                        match operator {
                            '+' => { return Value::Float(l as f64 + r) },
                            '-' => { return Value::Float(l as f64 - r) },
                            '*' => { return Value::Float(l as f64 * r) },
                            '/' => { return Value::Float(l as f64 / r) },
                            '%' => { return Value::Float(l as f64 % r) },
                            '^' => { return Value::Float((l as f64).powf(r)) },
                            _ => panic!(),
                        }
                    },
                    Value::OString(r) => {
                        match operator {
                            '+' => {
                                let mut l_str: String = l.to_string();
                                l_str.push_str(r.as_str());
                                return Value::OString(l_str)
                            },
                            _ => panic!(),
                        }
                    }
                }
            },
            Value::Float(l) => {
                match right_val {
                    Value::Number(r) => {
                        match operator {
                            '+' => { return Value::Float(l+ r as f64) },
                            '-' => { return Value::Float(l- r as f64) },
                            '*' => { return Value::Float(l * r as f64 ) },
                            '/' => { return Value::Float(l / r as f64 ) },
                            '%' => { return Value::Float(l % r as f64 ) },
                            '^' => { return Value::Float((l).powf(r as f64)) },
                            _ => panic!(),
                        }
                    },
                    Value::Float(r) => {
                        match operator {
                            '+' => { return Value::Float(l+ r) },
                            '-' => { return Value::Float(l- r) },
                            '*' => { return Value::Float(l * r ) },
                            '/' => { return Value::Float(l / r ) },
                            '%' => { return Value::Float(l % r ) },
                            '^' => { return Value::Float((l).powf(r)) },
                            _ => panic!(),
                        }
                    },
                    Value::OString(r) => {
                        match operator {
                            '+' => {
                                let mut l_str: String = l.to_string();
                                l_str.push_str(r.as_str());
                                return Value::OString(l_str)
                            },
                            _ => panic!(),
                        }
                    }
                }
            },
            Value::OString(l) => {
                match right_val {
                    Value::Number(r) => {
                        match operator {
                            '+' => {
                                let mut r_str: String = r.to_string();
                                r_str.push_str(l.as_str());
                                return Value::OString(r_str)
                            },
                            _ => panic!(),
                        }
                    },
                    Value::Float(r) => {
                        match operator {
                            '+' => {
                                let mut r_str: String = r.to_string();
                                r_str.push_str(l.as_str());
                                return Value::OString(r_str)
                            },
                            _ => panic!(),
                        }
                    },
                    Value::OString(r) => {
                        match operator {
                            '+' => {
                                let mut r_str: String = r.to_string();
                                r_str.push_str(l.as_str());
                                return Value::OString(r_str)
                            },
                            _ => panic!(),
                        }
                    }
                }
            },
        }
    }
}

