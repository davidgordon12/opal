use std::{env, collections::VecDeque};
use crate::compiler::Compiler;

mod compiler;

fn error(message: &str, file: &str) {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{}: \x1b[93m{}\x1b[0m", message, file);
}

fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    if args.len() == 1 {
        error("No input files provided", "");
        return;
    }

    // Compilation does not start until each file is validated (aka is a .opal file)
    //
    // We do not want to dump a bunch of warnings on the user at once.
    // Stop after we find the first offender.

    args.pop_front();

    for x in &args {
        if let Some(idx) = x.find('.') {
            let file_type = x.split_at(idx).1;
            if file_type != ".opal" { 
                error("Invalid file type", &x);
                break;
            }
        } else {
            error("Invalid file type", &x);
            break;
        }
    }


    let compiler: Compiler = Compiler {
        source: "".to_string(),
    };

    compiler.run();
}
