use std::io;
use std::io::prelude::*;

fn execute0(stack: &mut Vec<i128>, op: String) {
    let ret : i128;

    if op == "p" {
        ret = -1;
    } else if op == "n" {
        ret = -1;
    } else if op == "f" {
        ret = -1;
    } else {
        panic!("! Error: execute0() was called with op={}, this should never happen.", op);
    }
}

fn execute1(stack: &mut Vec<i128>, op: String, b: i128) {
    if op == "P" {
        panic!("P not implemented.");
    } else {
        panic!("! Error: execute1() was called with op={}, this should never happen.", op);
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
        panic!("! Error: execute2() was called with op={}, this should never happen.", op);
    }

    stack.push(ret);
    println!("> {}", ret);
}

fn execute(stack: &mut Vec<i128>, op: String) {
    if let Ok(n) = op.parse::<i128>() {
        stack.push(n);
        return;
    }

    let ret : i128;

    // Functions that do not modify the stack.
    if op == "p" || op == "n" || op == "f" {
        execute0(stack, op);
        return;
    }

    let b;
    if let Some(b2) = stack.pop() {
        b = b2;
    } else {
        println!("! Error: Not enough items on stack for {}.", op);
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
        println!("! Error: Not enough items on stack for {}.", op);
        return;
    }

    // Binary functions.
    if op == "+" || op == "-" || op == "*" || op == "/" {
        execute2(stack, op, a, b);
        return;
    }

    println!("! Error: No such function: {}", op);
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
