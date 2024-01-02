use std::io::Write;
use std::process::Command;

use crate::ast::Program;
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
        self.create_asm();

        self.compile_binary_expr();

        self.exit();

        Command::new("make").output().unwrap();
    }

    fn compile_binary_expr(&self) {
        let expr = self.program.body[0].clone().unwrap_binary_expr();

        let op = expr.operator.as_bytes()[0 as usize] as char;
        let lhs = expr.left.unwrap_number();
        let rhs = expr.right.unwrap_number();
        match &op {
            '+' => self.add(lhs.value, rhs.value),      
            '-' => self.subtract(lhs.value, rhs.value),      
            '*' => self.multiply(lhs.value, rhs.value),      
            '/' => self.divide(lhs.value, rhs.value),      
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
