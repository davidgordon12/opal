use std::ops::Add;

use crate::{ast::Program, error::error};

pub struct Compiler {
    program: Program,
}

impl Compiler {
    pub fn new(program: Program) -> Self {
        Compiler { 
            program: program,
        }
    }

    pub fn run(&self) {

    }

    fn compile_binary_expr(&self) {
        let expr = self.program.body[0].clone().unwrap_binary_expr();

        let op = expr.operator[0 as usize];
        let lhs = expr.left.unwrap_number();
        let rhs = expr.right.unwrap_number();
        let mut val: f32 = 0.0;
        match &op {
            '+' => val = lhs.value + rhs.value,      
            _ => error("Failed", Some("")),
        }

        unreachable!()
    }
}
