use std::io::Write;

pub fn push_register(file_path: &str, register: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");
    let mut file = std::fs::File::options().append(true).write(true).open(path).unwrap();

    let mut arg: String = String::from("push ");
    arg.push_str(register);
    arg.push('\n');
    file.write(arg.as_bytes()).unwrap();
}

pub fn push_number(file_path: &str, n: i64) {
    let mut path = file_path.to_string();
    path.push_str(".t");
    let mut file = std::fs::File::options().append(true).write(true).open(path).unwrap();

    let mut arg: String = String::from("mov rax, ");
    arg.push_str(&n.to_string());
    arg.push('\n');
    file.write(arg.as_bytes()).unwrap();

    push_register(file_path, "rax");
}

pub fn pop_number(file_path: &str, register: &str) {
    let mut path = file_path.to_string();
    path.push_str(".t");
    let mut file = std::fs::File::options().append(true).write(true).open(path).unwrap();

    let mut arg: String = String::from("pop ");
    arg.push_str(register);
    arg.push('\n');
    file.write(arg.as_bytes()).unwrap();
}

/*
pub fn push_float(file_path: &str, f: f64) {

}

pub fn pop_float(file_path: &str, register: &str) {

}
*/
