use std::collections::HashMap;

use crate::runtime::values::Value;
use crate::{ast::*, error::*};

pub struct OVM {
    ast: Vec<Node>,
    constants: HashMap<String, Value>,
    stack: Vec<Value>,
    function_table: HashMap<String, ProcDeclaration>,
}

impl OVM {
    pub fn new(ast: Vec<Node>) -> OVM {
        OVM {
            ast: ast.clone(),
            constants: HashMap::new(),
            stack: Vec::new(),
            function_table: HashMap::new(),
        }
    }

    pub fn execute(&mut self) {
        let ast = self.ast.clone();
        self._execute(ast);
    }

    fn _execute(&mut self, body: Vec<Node>) {
        for x in body {
            match x {
                Node::LetDeclaration(x) => self.evaluate_let_decleration(x),
                Node::ProcDeclaration(x) => {
                    self.function_table.insert(x.identifier.clone(), x);
                }

                Node::PrintStatement(x) => self.evaluate_print_statement(x),
                Node::ReturnStatement(x) => {
                    self.evaluate_return_statement(x);
                    return;
                }
                Node::IfStatement(x) => self.evaluate_if_statement(x),

                Node::ProcedureCall(x) => self.evaluate_procedure_call(x),
                _ => panic!(),
            }
        }
    }

    fn add_constant(&mut self, key: String, value: Value) {
        match value {
            Value::OString(x) => {
                self.constants.insert(key, Value::OString(x));
            }
            Value::Number(x) => {
                self.constants.insert(key, Value::Number(x));
            }
            Value::Float(x) => {
                self.constants.insert(key, Value::Float(x));
            }
            Value::Boolean(x) => {
                self.constants.insert(key, Value::Boolean(x));
            }
        }
    }

    fn get_constant(&mut self, key: String) -> Value {
        let var = self.constants.get(&key);
        if var.is_none() {
            opal_error_vm_invalid_variable(key);
        }
        var.unwrap().clone()
    }

    fn evaluate_procedure_call(&mut self, caller: ProcedureCall) {
        let proc_option = self.function_table.get(&caller.caller.symbol);

        if proc_option.is_none() {
            return;
        }

        let proc = proc_option.unwrap().clone();
        for node in proc.body {
            match node {
                Node::LetDeclaration(node) => self.evaluate_let_decleration(node),
                Node::PrintStatement(node) => self.evaluate_print_statement(node),
                Node::ProcedureCall(x) => self.evaluate_procedure_call(x),
                Node::PrintStatement(x) => self.evaluate_print_statement(x),
                Node::ReturnStatement(x) => self.evaluate_return_statement(x),
                Node::IfStatement(x) => self.evaluate_if_statement(x),
                _ => panic!("panicked"),
            }
        }
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
            }
            _ => panic!(),
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
                    Value::Boolean(x) => println!("{}", x),
                }
            }
            Node::Identifier(x) => {
                let value = self.get_constant(x.symbol);
                match value {
                    Value::OString(x) => println!("{}", x),
                    Value::Number(x) => println!("{}", x),
                    Value::Float(x) => println!("{}", x),
                    Value::Boolean(x) => println!("{}", x),
                }
            }
            _ => panic!(),
        }
    }

    fn evaluate_if_statement(&mut self, stmt: IfStatement) {
        match *stmt.expr {
            Node::BinaryExpr(x) => {
                let val = self.evaluate_binary_expression(x);
                match val {
                    Value::Boolean(x) => {
                        if x == true {
                            self._execute(stmt.body);
                        } else {
                            return;
                        }
                    }
                    _ => panic!(),
                }
            }
            Node::ProcedureCall(x) => { /* did the proc return true or false */ }
            _ => panic!(),
        }
    }

    fn evaluate_return_statement(&mut self, stmt: ReturnStatement) {
        if stmt.value.is_none() {
            return;
        }
        match *stmt.value.unwrap() {
            Node::Float(x) => {
                self.stack.push(Value::Float(x.value));
            }
            Node::Number(x) => {
                self.stack.push(Value::Number(x.value));
            }
            Node::OString(x) => {
                self.stack.push(Value::OString(x.value));
            }
            Node::Boolean(x) => {
                self.stack.push(Value::Boolean(x.value));
            }
            Node::Identifier(x) => {
                let val = self.get_constant(x.symbol);
                self.stack.push(val);
            }
            _ => panic!(),
        }
    }

    fn evaluate_binary_expression(&mut self, expr: BinaryExpr) -> Value {
        let lhs = *expr.left;

        match lhs {
            Node::Number(x) => {
                self.stack.push(Value::Number(x.value));
            }
            Node::Float(x) => {
                self.stack.push(Value::Float(x.value));
            }
            Node::OString(x) => {
                self.stack.push(Value::OString(x.value));
            }
            Node::BinaryExpr(x) => {
                let val = self.evaluate_binary_expression(x);
                self.stack.push(val)
            }
            Node::Identifier(x) => {
                let val = self.get_constant(x.symbol.clone());
                match val {
                    Value::OString(x) => {
                        self.stack.push(Value::OString(x));
                    }
                    Value::Number(x) => {
                        self.stack.push(Value::Number(x));
                    }
                    Value::Float(x) => {
                        self.stack.push(Value::Float(x));
                    }
                    Value::Boolean(x) => {
                        self.stack.push(Value::Boolean(x));
                    }
                }
            }
            _ => {
                opal_error_vm_invalid_expr();
                unreachable!()
            }
        }

        let rhs = *expr.right;

        match rhs {
            Node::Number(x) => {
                self.stack.push(Value::Number(x.value));
            }
            Node::Float(x) => {
                self.stack.push(Value::Float(x.value));
            }
            Node::OString(x) => {
                self.stack.push(Value::OString(x.value));
            }
            Node::BinaryExpr(x) => {
                let val = self.evaluate_binary_expression(x);
                self.stack.push(val)
            }
            Node::Identifier(x) => {
                let val = self.get_constant(x.symbol.clone());
                match val {
                    Value::OString(x) => {
                        self.stack.push(Value::OString(x));
                    }
                    Value::Number(x) => {
                        self.stack.push(Value::Number(x));
                    }
                    Value::Float(x) => {
                        self.stack.push(Value::Float(x));
                    }
                    Value::Boolean(x) => {
                        self.stack.push(Value::Boolean(x));
                    }
                }
            }
            _ => {
                opal_error_vm_invalid_expr();
                unreachable!()
            }
        }

        /* Horrifying code, but a brighter way to solve this hasn't come to me yet */
        let operator = expr.operator;

        let right_val = self.stack.pop().unwrap();
        let left_val = self.stack.pop().unwrap();

        match left_val {
            Value::Number(l) => match right_val {
                Value::Number(r) => match operator.as_str() {
                    "+" => return Value::Number(l + r),
                    "-" => return Value::Number(l - r),
                    "*" => return Value::Number(l * r),
                    "/" => return Value::Float(l as f64 / r as f64),
                    "%" => return Value::Float(l as f64 % r as f64),
                    "^" => return Value::Float((l as f64).powf(r as f64)),
                    ">" => return Value::Boolean(l > r),
                    ">=" => return Value::Boolean(l >= r),
                    "<" => return Value::Boolean(l < r),
                    "<=" => return Value::Boolean(l <= r),
                    "!=" => return Value::Boolean(l != r),
                    "==" => return Value::Boolean(l == r),
                    _ => panic!(),
                },
                Value::Float(r) => match operator.as_str() {
                    "+" => return Value::Float(l as f64 + r),
                    "-" => return Value::Float(l as f64 - r),
                    "*" => return Value::Float(l as f64 * r),
                    "/" => return Value::Float(l as f64 / r),
                    "%" => return Value::Float(l as f64 % r),
                    "^" => return Value::Float((l as f64).powf(r)),
                    ">" => return Value::Boolean(l as f64 > r),
                    ">=" => return Value::Boolean(l as f64 >= r),
                    "<" => return Value::Boolean((l as f64) < r),
                    "<=" => return Value::Boolean(l as f64 <= r),
                    "!=" => return Value::Boolean(l as f64 != r),
                    "==" => return Value::Boolean(l as f64 == r),
                    _ => panic!(),
                },
                Value::OString(r) => match operator.as_str() {
                    "+" => {
                        let mut l_str: String = l.to_string();
                        l_str.push_str(r.as_str());
                        return Value::OString(l_str);
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Value::Float(l) => match right_val {
                Value::Number(r) => match operator.as_str() {
                    "+" => return Value::Float(l + r as f64),
                    "-" => return Value::Float(l - r as f64),
                    "*" => return Value::Float(l * r as f64),
                    "/" => return Value::Float(l / r as f64),
                    "%" => return Value::Float(l % r as f64),
                    "^" => return Value::Float((l).powf(r as f64)),
                    ">" => return Value::Boolean(l > r as f64),
                    ">=" => return Value::Boolean(l >= r as f64),
                    "<" => return Value::Boolean(l < r as f64),
                    "<=" => return Value::Boolean(l <= r as f64),
                    "!=" => return Value::Boolean(l != r as f64),
                    "==" => return Value::Boolean(l == r as f64),
                    _ => panic!(),
                },
                Value::Float(r) => match operator.as_str() {
                    "+" => return Value::Float(l + r),
                    "-" => return Value::Float(l - r),
                    "*" => return Value::Float(l * r),
                    "/" => return Value::Float(l / r),
                    "%" => return Value::Float(l % r),
                    "^" => return Value::Float((l).powf(r)),
                    ">" => return Value::Boolean(l > r),
                    ">=" => return Value::Boolean(l >= r),
                    "<" => return Value::Boolean(l < r),
                    "<=" => return Value::Boolean(l <= r),
                    "!=" => return Value::Boolean(l != r),
                    "==" => return Value::Boolean(l == r),
                    _ => panic!(),
                },
                Value::OString(r) => match operator.as_str() {
                    "+" => {
                        let mut l_str: String = l.to_string();
                        l_str.push_str(r.as_str());
                        return Value::OString(l_str);
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Value::OString(l) => match right_val {
                Value::Number(r) => match operator.as_str() {
                    "+" => {
                        let mut r_str: String = r.to_string();
                        r_str.push_str(l.as_str());
                        return Value::OString(r_str);
                    }
                    _ => panic!(),
                },
                Value::Float(r) => match operator.as_str() {
                    "+" => {
                        let mut r_str: String = r.to_string();
                        r_str.push_str(l.as_str());
                        return Value::OString(r_str);
                    }
                    _ => panic!(),
                },
                Value::OString(r) => match operator.as_str() {
                    "+" => {
                        return Value::OString(l + &r);
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
