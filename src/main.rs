use std::process;

use parallel_factorial::factorial;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("Please supply an argument.");
        process::exit(22); // EINVAL
    };
    let n: u64 = args[1].parse().expect("Malformed number.");
    println!("{}", factorial(n));
}
