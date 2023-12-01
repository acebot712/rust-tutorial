use std::time::Instant;

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    let start = Instant::now();
    let n: u64 = 20; // You can change this to a larger number
    let result = factorial(n);
    let duration = start.elapsed();
    
    println!("Factorial of {} is: {}", n, result);
    println!("Time taken: {:?}", duration);
}
