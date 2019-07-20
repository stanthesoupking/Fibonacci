/**
 * Generates the given number in the Fibonacci Sequence
 * 
 * Author: Stanley Fuller
 */

use std::io;

fn main() {
    println!("What fibonacci number would you like to generate?");

    let n: usize;
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed reading line.");
        n = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: '{}' is not a valid integer (>= 0).", input.trim());
                continue;
            }
        };
        break;
    }

    let result = fib(n);

    println!("Result is: {}", result);
}

fn fib(n: usize) -> usize {
    let mut m: Vec<usize> = vec![1,1];

    // Do we need to generate further values
    if n > 1 {
        for i in 1..n {
            // Add previous 2 indicies
            let v = m[i] + m[i - 1];

            m.push(v);
        }
    }

    // Return final fib number
    m[n]
}
