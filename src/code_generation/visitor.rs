use crate::ast::{Program, Stmt, BinaryExpr, LetDeclaration};
use crate::error::error;

use crate::code_generation::stack::push_number;
use crate::code_generation::binop::*;
use crate::code_generation::bootstrap::*;

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
        create_text_section(&self.file_path);
        create_data_section(&self.file_path);
        create_bss_section(&self.file_path);

        /* Walk the tree here and determine the compilation method for the specific node */
        for stmt in self.program.body.clone() {
            match stmt {
                Stmt::BinaryExpr(x) => self.generate_binary_expr(x),
                Stmt::LetDeclaration(x) => self.generate_let_decl(x),
                _ => {},
            }
        }

        let _ = concat(&self.file_path);
    }

    fn generate_binary_expr(&self, expr: BinaryExpr) {
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

    fn generate_let_decl(&self, decl: LetDeclaration) {
        let ident = decl.identifier;
        let val = *decl.value;

        match val {
            Stmt::Number(x) => println!("Variable \"{}\" has a value of `{:#?}`.", ident, x.value),
            Stmt::BinaryExpr(x) => println!("Variable \"{}\" has a value of `{:#?}`.", ident, x.operator),
            _ => unimplemented!()
        }
    }
}
