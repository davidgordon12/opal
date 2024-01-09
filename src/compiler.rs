use std::io::Write;

use crate::ast::{Program, Stmt, BinaryExpr};
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
        let _ = std::fs::remove_file(&self.file_path);
        let mut file = std::fs::File::options().append(true).create(true).open(&self.file_path).unwrap();
        file.write(b"section .text\n").unwrap();
        file.write(b"global _start\n").unwrap();
        file.write(b"\n_start:").unwrap();
    }

    pub fn run(&mut self) {
        /* This method will walk the AST and compile the source code into NASM assembly for the x86_64 architecture */
        self.create_asm();

        /* Walk the tree here and determine the compilation method for the specific node */
        for stmt in self.program.body.clone() {
            match stmt {
                Stmt::BinaryExpr(x) => self.compile_binary_expr(x),
                _ => {},
            }
        }

        self.exit();
    }

    fn compile_binary_expr(&self, expr: BinaryExpr) {
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
            Stmt::Number(n) => self.push_number(n.value),
            Stmt::BinaryExpr(x) => self.compile_binary_expr(x),
            _ => unimplemented!()
        }

        let rhs = *expr.right;

        match rhs {
            Stmt::Number(n) => self.push_number(n.value),
            Stmt::BinaryExpr(x) => self.compile_binary_expr(x),
            _ => unimplemented!()
        }

        let op = expr.operator;
        match &op {
            '+' => self.add(),
            '-' => self.subtract(),
            '*' => self.multiply(),
            '/' => self.divide(),
            _ => error("Illegal operator", None),
        }
    }

    fn push_number(&self, n: i64) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("mov rax, ");
        arg.push_str(&n.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        file.write(b"\n        ").unwrap();
        file.write(b"push rax").unwrap();
    }

    fn pop_number(&self, register: &str) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        let mut arg: String = String::from("pop ");
        arg.push_str(register);
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn add(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        self.pop_number("rbx");
        self.pop_number("rax");

        let arg: String = String::from("add rax, rbx\n        push rax");

        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn subtract(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        self.pop_number("rbx");
        self.pop_number("rax");

        let arg: String = String::from("sub rax, rbx\n        push rax");

        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }
    
    fn multiply(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        self.pop_number("rbx");
        self.pop_number("rax");

        let arg: String = String::from("mul rbx\n        push rax");

        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn divide(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        self.pop_number("rbx");
        self.pop_number("rax");

        let arg: String = String::from("div rbx\n        push rax");

        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }

    fn exit(&self) {
        let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

        file.write(b"\n        ").unwrap();
        file.write(b"jmp exit").unwrap();

        file.write(b"\n        ").unwrap();
        file.write(b"ret").unwrap();

        let arg: String = String::from("\nexit:");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

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
