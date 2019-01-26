use std::env;
use std::io;
use std::io::prelude::*;

fn print_help() {
    println!("Numbers are always pushed to the stack.");
    println!("");
    println!("Available functions:");
    println!("  q       exit immediately");
    println!("  p       print the top item on the stack.");
    println!("  P       pop the top item off the stack, then print it.");
    println!("  f       print all items on the stack.");
    println!("  +       pop two items off the stack, add them, push the result.");
    println!("  -       pop two items (a, b) off the stack, subtract a from b, push the result.");
    println!("  /       pop two items (a, b) off the stack, divide b by a, push the result.");
    println!("  *       pop two items off the stack, multiply them, push the result.");
}

fn execute0(stack: &mut Vec<i128>, op: String) {
    if op == "q" {
        std::process::exit(0);
    } else if op == "p" {
        if stack.len() == 0 {
            println!("!  Stack is empty.");
            return;
        }
        println!(" > {}", stack[stack.len() - 1]);
    } else if op == "f" {
        if stack.len() == 0 {
            println!("!  Stack is empty.");
            return;
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

    if op == "h" || op == "help" { print_help(); return; }

    // Functions that do not modify the stack.
    if op == "q" || op == "p" || op == "n" || op == "f" {
        execute0(stack, op);
        return;
    }

    let b;
    if let Some(b2) = stack.pop() {
        b = b2;
    } else {
        println!("!  Stack is empty.");
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
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage, interactive: chewy");
        println!("   non-interactive: echo \"1 2 3 + *\" | chewy");
        println!("");
        print_help();
        std::process::exit(0);
    }


    let mut stack: Vec<i128> = Vec::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        for word in line.unwrap().split_whitespace() {
            execute(&mut stack, word.to_string());
        }
    }
}
