pub fn error(message: &str, error: Option<&str>) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{} \x1b[93m{}\x1b[0m", message, error.unwrap_or(""));
    std::process::exit(1);
}
