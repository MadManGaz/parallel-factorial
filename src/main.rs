use std::process;

use rayon::prelude::*;
use rug::Integer;

fn parallel_factorial(n: u64) -> String {
    String::from("")
}

fn factorial(n: u64) -> String {
    let mut acc = Integer::from(n);
    for index in 1..n {
        acc *= index;
    }
    acc.to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("Please supply an argument.");
        process::exit(22); // EINVAL
    };
    let n: u64 = args[1].parse().expect("Malformed number");
    println!("factorial({}) = {}", n, factorial(n));
}
