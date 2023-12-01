fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n = 17;
    if is_prime(n) {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }
}
