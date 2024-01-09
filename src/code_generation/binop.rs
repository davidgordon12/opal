use std::io::Write;

use crate::code_generation::stack::pop_number;

use super::stack::push_register;

pub fn add(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");
    
    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    pop_number(file_path, "rbx");
    pop_number(file_path, "rax");

    file.write(b"add rax, rbx\n").unwrap();
    push_register(file_path, "rax");
}

pub fn subtract(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");
    
    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    pop_number(file_path, "rbx");
    pop_number(file_path, "rax");

    file.write(b"sub rax, rbx\n").unwrap();
    push_register(file_path, "rax");
}
 
pub fn multiply(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");

    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    pop_number(file_path, "rbx");
    pop_number(file_path, "rax");

    file.write(b"mul rbx\n").unwrap();
    push_register(file_path, "rax");
}

pub fn divide(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");

    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    pop_number(file_path, "rbx");
    pop_number(file_path, "rax");

    file.write(b"div rbx\n").unwrap();
    push_register(file_path, "rax");
}