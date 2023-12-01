fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    let n = 10;
    for i in 0..n{
        println!("Fibonacci({})={}", i, fibonacci(i));
    }
}