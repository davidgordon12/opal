pub fn error(message: &str, error: Option<&str>) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{} \x1b[93m{}\x1b[0m", message, error.unwrap_or(""));
    std::process::exit(1);
}

pub fn parse_token_error(message: &str, token_literal: &str, line: &str) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{} \x1b[93m{}\x1b[0m on line \x1b[93m{}\x1b[0m", message, token_literal, line);
    std::process::exit(1);
}