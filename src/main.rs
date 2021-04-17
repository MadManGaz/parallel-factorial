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
    if args.is_empty() {
        panic!("Failed to supply argument.");
    }
    let n: u64 = args[0].parse().expect("Malformed number");
    println!("factorial({}) = {}", n, factorial(n));
}
