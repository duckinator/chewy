use std::io;
use std::io::prelude::*;

fn execute_operation(op: String, a: i128, b: i128) -> Result<i128, String> {
    let ret : i128;

    if op == "+" {
        ret = b + a;
    } else if op == "-" {
        ret = b - a;
    } else if op == "*" {
        ret = b * a;
    } else if op == "/" {
        ret = b / a;
    } else {
        return Err(format!("No such function: {}", op));
    }

    return Ok(ret)
}

fn execute(stack: &mut Vec<i128>, line: String) {
    if let Ok(n) = line.parse::<i128>() {
        stack.push(n);
        return;
    }

    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
        match execute_operation(line, a, b) {
            Ok(ret) => {
                stack.push(ret);
                println!("> {}", ret);
            },
            Err(msg) => println!("! Error: {}", msg),
        }
    } else {
        println!("! Error: Not enough items on stack.");
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
