use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::ast::{Program, Expr, BinaryExpr};
use crate::error::error;

pub struct Compiler {
    program: Program,
    file_path: String,
}

impl Compiler {
    pub fn new(file: String, program: Program) -> Self {
        let mut path = file.clone();
        path.push_str(".asm");
        Compiler { 
            program: program,
            file_path: path,
        }
    }

    pub fn create_asm(&self) {
        let mut file = std::fs::File::create(&self.file_path).unwrap();
        file.write(b"section .text
global _start

_start:"
        ).unwrap();
    }

    pub fn run(&mut self) {
        /* This method will walk the AST and compile the source code into NASM assembly for the x86_64 architecture */
        self.create_asm();

        /* Walk the tree here and determine the compilation method for the specific node */
        for x in &self.program.body {
            self.compile_binary_expr(x.clone().unwrap_binary_expr());
        }
        self.exit();
    }

    fn compile_binary_expr(&self, expr: BinaryExpr) {
        let mut lhs = 0.0;
        let mut rhs = 0.0;

        let left = expr.left;
        match *left {
            Expr::BinaryExpr(x) => self.compile_binary_expr(x), 
            Expr::Number(x) => lhs = x.value,
            _ => {},
        }

        let right = expr.right;
        match *right {
            Expr::BinaryExpr(x) => self.compile_binary_expr(x), 
            Expr::Number(x) => rhs = x.value,
            _ => {},
        }

        let op = expr.operator.as_bytes()[0 as usize] as char;
        match &op {
            '+' => self.add(lhs, rhs),      
            '-' => self.subtract(lhs, rhs),      
            '*' => self.multiply(lhs, rhs),      
            '/' => self.divide(lhs, rhs),      
            _ => error("Illegal operator", None),
        }
    }

    fn add(&self, a: f64, b: f64) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("mov rax, ");
        arg.push_str(&a.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let mut arg: String = String::from("mov rbx, ");
        arg.push_str(&b.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("add rax, rbx");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn subtract(&self, a: f64, b:f64) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("mov rax, ");
        arg.push_str(&a.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let mut arg: String = String::from("mov rbx, ");
        arg.push_str(&b.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("sub rax, rbx");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn multiply(&self, a: f64, b:f64) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("mov rax, ");
        arg.push_str(&a.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let mut arg: String = String::from("mov rbx, ");
        arg.push_str(&b.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("mul rbx");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn divide(&self, a: f64, b:f64) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("mov rax, ");
        arg.push_str(&a.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let mut arg: String = String::from("mov rbx, ");
        arg.push_str(&b.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("div rbx");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn exit(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        file.write(b"\n        ").unwrap();

        let arg: String = String::from("mov rax, 1");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("mov rbx, 0");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("int 0x80");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }
}
