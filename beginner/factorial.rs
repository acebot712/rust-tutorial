fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    return n * factorial(n - 1);
}


fn main(){
    let n = 5;
    println!("Factorial({})={}",n,factorial(n));
}