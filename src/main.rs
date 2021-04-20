use std::process;

use mp_factorial::parallel_factorial;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("Please supply an argument.");
        process::exit(22); // EINVAL
    };
    let n: u64 = args[1].parse().expect("Malformed number.");
    println!("{}", parallel_factorial(n));
}
