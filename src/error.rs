// I don't know how to write a proper macro to do this for me
// so I am just going to create a method per common error

use std::process::exit;

fn print_error_header() {
    print!("\x1b[1m \x1b[91mERROR! \x1b[0m");
}

fn print_error_type(err_type: &str, arg: Option<String>) {
    if let Some(val) = arg {
        print!("{} ", err_type);
        println!("'\x1b[1m\x1b[91m{}\x1b[0m'.", val);
    } else {
        println!("{}.", err_type);
    }
}

pub fn opal_error_invalid_args() {
    print_error_header();
    print_error_type("Invalid arguments to opalc", None);
    println!("Usage: opalc <filename>");
    exit(1)
}

pub fn opal_error_invalid_file_type(file_name: String) {
    print_error_header();
    print_error_type("Incorrect file type", Some(file_name));
    println!("File must end with '.opal'");
    exit(1)
}

pub fn opal_error_file_not_exists(file_name: String) {
    print_error_header();
    print_error_type("File does not exist", Some(file_name));
    exit(1)
}
