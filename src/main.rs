use std::path::Path;
use std::{collections::VecDeque, env};

use error::*;
use opal::opalc;

mod ast;
mod error;
mod lexer;
mod opal;
mod parser;
mod tokens;

mod runtime;

fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    if args.len() == 1 {
        opal_error_invalid_args();
    }

    // Compilation does not start until each file is validated (aka is a .opal file)
    //
    // We do not want to dump a bunch of warnings on the user at once.
    // Stop after we find the first offender.

    // First argument is always the path to the executable
    args.pop_front();

    // First check if the file exists
    for x in &args {
        if !Path::new(&x).exists() {
            opal_error_file_not_exists(x.to_string());
        }
    }

    // Then check if it is a valid file
    for x in &args {
        // This will fail if a file has multiple '.'s, but we will choose not to support that for the time being
        if let Some(idx) = x.find('.') {
            if x.split_at(idx).1 != ".opal" {
                opal_error_invalid_file_type(x.to_string());
            }
        } else {
            // If the path doesn't contain a . at all then report it
            opal_error_invalid_file_type(x.to_string());
        }
    }

    // Begin compilation
    opalc(args)
}
