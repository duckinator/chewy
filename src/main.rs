use std::io;
use std::io::prelude::*;

fn execute0(stack: &mut Vec<i128>, op: String) {
    let ret : i128;

    if op == "q" {
        std::process::exit(0);
    } else if op == "p" {
        println!(" > {}", stack[stack.len() - 1]);
    } else if op == "f" {
        if stack.len() == 0 {
            println!("!   Stack is empty.");
        }

        for i in 0..(stack.len() - 1) {
            println!(" ~ {}", stack[i]);
        }
        println!(" > {}", stack[stack.len() - 1]);
    } else {
        panic!("!   execute0() was called with op={}, this should never happen.", op);
    }
}

fn execute1(stack: &mut Vec<i128>, op: String, b: i128) {
    if op == "P" {
        println!("=  {}", b);
    } else {
        panic!("!   execute1() was called with op={}, this should never happen.", op);
    }
}

fn execute2(stack: &mut Vec<i128>, op: String, a: i128, b: i128) {
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
        panic!("!  execute2() was called with op={}, this should never happen.", op);
    }

    stack.push(ret);
    println!("=> {}", ret);
}

fn execute(stack: &mut Vec<i128>, op: String) {
    if let Ok(n) = op.parse::<i128>() {
        stack.push(n);
        return;
    }

    let ret : i128;

    // Functions that do not modify the stack.
    if op == "q" || op == "p" || op == "n" || op == "f" {
        execute0(stack, op);
        return;
    }

    let b;
    if let Some(b2) = stack.pop() {
        b = b2;
    } else {
        println!("!  Not enough items on stack.");
        return;
    }


    // Unary functions.
    if op == "P" {
        execute1(stack, op, b);
        return;
    }

    let a;
    if let Some(a2) = stack.pop() {
        a = a2;
    } else {
        println!("!  Not enough items on stack.");
        return;
    }

    // Binary functions.
    if op == "+" || op == "-" || op == "*" || op == "/" {
        execute2(stack, op, a, b);
        return;
    }

    println!("!  No such function: {}", op);
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
