pub fn print_help() {
    println!("Numbers are always pushed to the stack.");
    println!("");
    println!("Available functions:");
    println!("  q       exit immediately.");
    println!("  V       print version information.");
    println!("  p       print the top item on the stack.");
    println!("  P       pop the top item off the stack, then print it.");
    println!("  f       print all items on the stack.");
    println!("  +       pop two items off the stack, add them, push the result.");
    println!("  -       pop two items (a, b) off the stack, subtract a from b, push the result.");
    println!("  /       pop two items (a, b) off the stack, divide b by a, push the result.");
    println!("  *       pop two items off the stack, multiply them, push the result.");
}

/// Operations with no stack changes.
fn execute0(stack: &mut Vec<i128>, op: String) {
    if op == "q" {
        std::process::exit(0);
    } else if op == "h" || op == "help" {
        print_help();
    } else if op == "V" {
        // NOTE: The leading 3 spaces are to correspond with the
        //       "<command><severity indicator><space><output>" format
        //       convention.
        println!("   {} v{}",
                 env!("CARGO_PKG_NAME"),
                 env!("CARGO_PKG_VERSION"));
    } else if op == "p" {
        if stack.len() == 0 {
            println!("p! Stack is empty.");
            return;
        }
        println!("p  {}", stack[stack.len() - 1]);
    } else if op == "f" {
        if stack.len() == 0 {
            println!("f! Stack is empty.");
            return;
        }

        print!("f ");
        for n in stack {
            print!(" {}", n);
        }
        println!("");
    } else {
        panic!("{}! execute0() was called with op={}, this should never happen.", op, op);
    }
}

/// Unary operations. (One item is removed from the stack.)
fn execute1(_stack: &mut Vec<i128>, op: String, b: i128) {
    // The removal from the stack happens in `execute_one()`,
    // so we never need to touch `_stack` here.
    if op == "P" {
        println!("P> {}", b);
    } else {
        panic!("P! execute1() was called with op={}, this should never happen.", op);
    }
}

/// Binary operations. (Two items are removed from the stack.)
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
        panic!("{}! execute2() was called with op={}, this should never happen.", op, op);
    }

    stack.push(ret);
    println!("{}  {} {}", op, a, b);
    println!("=> {}", ret);
}

/// Executes a single operation.
pub fn execute_one(stack: &mut Vec<i128>, op: String) {
    if let Ok(n) = op.parse::<i128>() {
        println!("<< {}", n);
        stack.push(n);
        return;
    }

    // Functions that do not modify the stack.
    if op == "h" || op == "help" || op == "q" || op == "V" || op == "p" || op == "n" || op == "f" {
        execute0(stack, op);
        return;
    }

    let b;
    if let Some(b2) = stack.pop() {
        b = b2;
    } else {
        println!(" ! Stack is empty.");
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
        println!(" ! Not enough items on stack.");
        return;
    }

    // Binary functions.
    if op == "+" || op == "-" || op == "*" || op == "/" {
        execute2(stack, op, a, b);
        return;
    }

    println!("{}! No such function: {}", op, op);
}

/// Execute all operations in a string, in order.
///
/// # Examples
///
/// ```
/// use chewy::execute;
///
/// let mut stack: Vec<i128> = Vec::new();
///
/// // 1. Push 1, 2, 3, 4 to the stack
/// // 2. Pop 4 and 3 off the stack, calculate 4 + 3, and push the result (7)
/// // 3. Pop 7 and 2 off the stack, calculate 7 + 2, and push the result (9)
/// // 4. Pop 9 and 1 off the stack, calculate 9 + 1, and push the result (10)
/// execute(&mut stack, "1 2 3 4 + + +");
/// assert_eq!(stack.last(), Some(&10));
///
/// // 5. Push 12 to the stack
/// // 6. Pop 12 and 10 off the stack, calculate 12 * 10, and push the result (120)
/// execute(&mut stack, "12 *");
/// assert_eq!(stack.last(), Some(&120));
///
/// // 7. Push 10 to the stack
/// execute(&mut stack, "10");
/// assert_eq!(stack.last(), Some(&10));
/// // 8. Pop 10 off the stack (leaving 120 as the result)
/// execute(&mut stack, "P");
/// assert_eq!(stack.last(), Some(&120));
/// ```
pub fn execute(stack: &mut Vec<i128>, line: &str) {
    for word in line.split_whitespace() {
        execute_one(stack, word.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::execute;

    fn execute_assert(input: &str, result: i128) {
        let mut stack: Vec<i128> = Vec::new();
        execute(&mut stack, input);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0], result);
    }

    #[test]
    fn addition1() {
        execute_assert("1 2 3 4 + + +", 10);
    }

    #[test]
    fn addition2() {
        execute_assert("1 2 3 + 4 + +", 10);
    }

    #[test]
    fn subtraction1() {
        execute_assert("1 2 3 4 - - -", ((4 - 3) - 2) - 1);
    }

    #[test]
    fn subtraction2() {
        execute_assert("1 2 3 - 4 - -", (4 - (3 - 2)) - 1);
    }

    #[test]
    fn multiplication1() {
        execute_assert("1 2 3 4 * * *", 4 * 3 * 2 * 1);
    }

    #[test]
    fn multiplication2() {
        execute_assert("1 2 3 * 4 * *", 4 * 3 * 2 * 1);
    }

    #[test]
    fn division1() {
        execute_assert("1 2 3 4 / / /", ((4 / 3) / 2) / 1);
    }

    #[test]
    fn division2() {
        execute_assert("1 2 3 / 4 / /", (4 / (3 / 2)) / 1);
    }

    #[test]
    fn complex() {
        execute_assert("1 2 * 4 - 3 +", 3 + (4 - (2 * 1)));
    }

}
