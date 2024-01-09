use crate::ast::{Program, Stmt, BinaryExpr};
use crate::error::error;

use crate::code_generation::stack::push_number;
use crate::code_generation::binop::*;
use crate::code_generation::assembly::*;

pub struct Visitor {
    program: Program,
    file_path: String,
}

impl Visitor {
    pub fn new(file: String, program: Program) -> Self {
        let mut path = file.clone();
        path.push_str(".asm");
        Visitor { 
            program: program,
            file_path: path,
        }
    }


    pub fn run(&mut self) {
        /* This method will walk the AST and generate the source code into NASM assembly for the x86_64 architecture */
        create_text_section(&self.file_path);
        create_data_section(&self.file_path);
        create_bss_section(&self.file_path);

        /* Walk the tree here and determine the compilation method for the specific node */
        for stmt in self.program.body.clone() {
            match stmt {
                Stmt::BinaryExpr(x) => self.generate_binary_expr(x),
                _ => {},
            }
        }

        let _ = concat(&self.file_path);
    }

    fn generate_binary_expr(&self, expr: BinaryExpr) {
        /* 
        * After every BinaryExpr, push to stack
        * We need to carry over some sort of flag,
        * to denote whether or not we need to pop for this
        * particular expression
        * If one arm is a BinaryExpr then pop one value
        * If both arms are BinaryExprs then pop twice 
        */
        let lhs = *expr.left;

        match lhs {
            Stmt::Number(n) => push_number(&self.file_path, n.value),
            Stmt::BinaryExpr(x) => self.generate_binary_expr(x),
            _ => unimplemented!()
        }

        let rhs = *expr.right;

        match rhs {
            Stmt::Number(n) => push_number(&self.file_path, n.value),
            Stmt::BinaryExpr(x) => self.generate_binary_expr(x),
            _ => unimplemented!()
        }

        let op = expr.operator;
        match &op {
            '+' => add(&self.file_path),
            '-' => subtract(&self.file_path),
            '*' => multiply(&self.file_path),
            '/' => divide(&self.file_path),
            _ => error("Illegal operator", None),
        }
    }
}
