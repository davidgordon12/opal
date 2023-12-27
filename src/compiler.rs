use std::{ops::Add, io::Bytes};

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
        self.compile_binary_expr()
    }

    fn compile_binary_expr(&self) {
        let expr = self.program.body[0].clone().unwrap_binary_expr();

        let op = expr.operator.as_bytes()[0 as usize] as char;
        let lhs = expr.left.unwrap_number();
        let rhs = expr.right.unwrap_number();
        let mut val: f32 = 0.0;
        match &op {
            '+' => val = lhs.value + rhs.value,      
            _ => error("Failed", Some("")),
        }

        println!("{}", val);

        panic!()
    }
}
