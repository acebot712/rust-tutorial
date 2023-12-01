fn main() {
    let mut nums: Vec<i32> = Vec::new();

    // Add 10 million integers to the array
    for i in 1..=10_000_000 {
        nums.push(i);
    }

    // Attempt to access an out-of-bounds element
    let _value = nums[10_000_001]; // This will cause a panic

    println!("This line won't be reached due to panic");
}
