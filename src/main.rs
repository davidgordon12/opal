use std::{env, collections::VecDeque};
use std::path::Path;

use opal::opalc;

mod opal;
mod tokens;
mod lexer;
mod parser;
mod ast;
mod compiler;

fn error(message: &str, file: &str) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{}: \x1b[93m{}\x1b[0m", message, file);
}

fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    if args.len() == 1 {
        error("No input files provided", "");
        return
    }

    // Compilation does not start until each file is validated (aka is a .opal file)
    //
    // We do not want to dump a bunch of warnings on the user at once.
    // Stop after we find the first offender.

    // First argument is always the path to the executable
    args.pop_front();

    for x in &args {
        // This will fail if a file has multiple '.'s, but we will choose not to support that for the time being
        if let Some(idx) = x.find('.') {
            if x.split_at(idx).1 != ".opal" { 
                error("Invalid file type", &x);
                return
            }
        } else {
            // If the path doesn't contain a . at all then report it
            error("Invalid file type", &x);
            return
        }
    }

    for x in &args {
        if !Path::new(&x).exists() {
            error("File does not exist", &x);
            return;
        }
    }

    opalc(args)
}
