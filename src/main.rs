use std::io;
use std::io::prelude::*;

fn execute(stack: &mut Vec<i128>, line: String) {
    if let Ok(n) = line.parse::<i128>() {
        stack.push(n);
    } else {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        let ret : i128;

        if line == "+" {
            ret = b + a;
        } else if line == "-" {
            ret = b - a;
        } else if line == "*" {
            ret = b * a;
        } else if line == "/" {
            ret = b / a;
        } else {
            println!("No such function: {}", line);
            return;
        }
        stack.push(ret);
        println!("> {}", ret);
    }
}

fn main() {
    let mut stack: Vec<i128> = Vec::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        for word in line.unwrap().split_whitespace() {
            execute(&mut stack, word.to_string());
        }
    }
}
