use std::env;
use std::io;
use std::io::prelude::*;

use chewy::execute;

fn main() -> Result<(), std::io::Error> {
    let mut stack: Vec<i128> = Vec::new();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage, interactive: chewy");
        println!("   non-interactive: echo \"1 2 3 + *\" | chewy");
        println!("");
        execute(&mut stack, "h q");
    }


    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        execute(&mut stack, &line?);
    }

    Ok(())
}
