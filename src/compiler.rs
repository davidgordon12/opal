use std::{io::Write, fs::File};

use crate::{ast::Program, error::error};

pub struct Compiler {
    file: String,
    program: Program,
}

impl Compiler {
    pub fn new(file: String, program: Program) -> Self {
        Compiler { 
            file: file,
            program: program,
        }
    }

    pub fn run(&mut self) {
        let mut path = self.file.clone();
        path.push_str(".asm");
        let mut file = std::fs::File::create(path).unwrap();
        file.write(b"section .text
global _start

_start:"
        ).unwrap();

        self.compile_binary_expr()
    }

    fn compile_binary_expr(&self) {
        let expr = self.program.body[0].clone().unwrap_binary_expr();

        let op = expr.operator.as_bytes()[0 as usize] as char;
        let lhs = expr.left.unwrap_number();
        let rhs = expr.right.unwrap_number();
        match &op {
            '+' => self.add(lhs.value, rhs.value),      
            _ => error("Illegal operator", None),
        }


        panic!()
    }

    fn add(&self, a: f32, b: f32) {
        let mut path = self.file.clone();
        path.push_str(".asm");
        let mut file = std::fs::File::options().write(true).append(true).open("tests/bin.opal.asm").unwrap();
        let mut arg: String = String::from("mov eax, ");
        arg.push_str(&a.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let mut arg: String = String::from("mov ebx, ");
        arg.push_str(&b.to_string());
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("add eax, ebx");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();

        let arg: String = String::from("ret eax");
        file.write(b"\n        ").unwrap();
        file.write(arg.as_bytes()).unwrap();
    }
}
