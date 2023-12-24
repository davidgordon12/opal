pub fn error(message: &str, line: Option<&str>, file: Option<&str>, token: Option<&str>) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{} \x1b[93m{}\x1b[0m", message, line.unwrap_or(file.unwrap_or(token.unwrap_or(""))));
    std::process::exit(1);
}
