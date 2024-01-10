use std::{io::Write, fs};

pub fn create_text_section(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");

    let _ = std::fs::remove_file(&path);
    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    file.write(b"section .text\n\n").unwrap();
    file.write(b"global main\n\n").unwrap();
    file.write(b"main:\n").unwrap();
}

pub fn create_data_section(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".d");

    let _ = std::fs::remove_file(&path);
    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    file.write(b"section .data\n").unwrap();
    file.write(b"fmt_digit db `%d\\n`\n").unwrap();
    file.write(b"fmt_char db `%c\\n`\n").unwrap();
    file.write(b"fmt_string db `%s\\n`\n").unwrap();
    file.write(b"fmt_float db `%f\\n`\n").unwrap();
}

pub fn create_bss_section(file_path: &str) {
    let mut path = file_path.to_string();
    path.push_str(".b");

    let _ = std::fs::remove_file(&path);
    let mut file = std::fs::File::options().append(true).create(true).open(path).unwrap();

    file.write(b"section .bss\n").unwrap();
}

fn exit(file_path: &str) {
    let mut text_path = file_path.to_string();
    text_path.push_str(".t");

    let mut text_file = std::fs::File::options().append(true).create(true).open(text_path).unwrap();

    text_file.write(b"mov rax, 60\n").unwrap();
    text_file.write(b"mov rdi, 0\n").unwrap();
    text_file.write(b"syscall\n").unwrap();
}

pub fn concat(file_path: &str) -> std::io::Result<()> {
    exit(file_path);

    let _ = std::fs::remove_file(file_path);

    let mut text_path = file_path.to_string();
    text_path.push_str(".t");

    let mut data_path = file_path.to_string();
    data_path.push_str(".d");

    let mut bss_path = file_path.to_string();
    bss_path.push_str(".b");

    let text = fs::read_to_string(&text_path)?;
    let data = fs::read_to_string(&data_path)?;
    let bss = fs::read_to_string(&bss_path)?;
    let result = text + "\n" + &data + "\n" + &bss;

    std::fs::remove_file(&text_path)?;
    std::fs::remove_file(&data_path)?;
    std::fs::remove_file(&bss_path)?;
    
    fs::write(&file_path, result)?;

    Ok(())
}
