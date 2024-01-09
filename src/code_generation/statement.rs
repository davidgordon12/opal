use crate::ast::Program;

pub fn print(file_path: &str, value_type: NodeType) {
    let mut file = std::fs::File::options().write(true).append(true).open(&self.file_path).unwrap();

    file.write(b"\n        ").unwrap();
    file.write(b"jmp print").unwrap();

    let arg: String = String::from("\nprint:");
    file.write(b"\n        ").unwrap();
    file.write(arg.as_bytes()).unwrap();

    file.write(b"
        pop rsi
        push rbp
        mov rbp, rsp
        lea rdi, [fmt]
        xor rax, rax
        call printf
        xor rax, rax
        leave"
    ).unwrap();
}