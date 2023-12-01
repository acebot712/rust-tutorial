# Rust Tutorial

Welcome to my Rust tutorial repository! Here, you'll find solutions I've written while problem-solving to improve my Rust language skills.

## Why Learn Rust for AI?

🤖 **AI Engineers, Boost Your Skills with Rust!** 🦀

- ⚙️ **Speed:** Rust's efficiency enhances AI performance.
- 🛡️ **Safety:** Memory safety guards your models.
- 🚀 **Concurrency:** Perfect for parallel AI tasks.

## Learning Resources

📚 To get started with Rust, check out the [Official Rust Book](https://doc.rust-lang.org/book/).

## Benchmarking Against Python

In this repository, you can find Python and Rust files for benchmarking against three different metrics:

1. **Performance**
2. **Memory Safety**
3. **Concurrency

Here are my findings:

### Table Summary for Benchmarking

| Metric            | **Python**                          | **Rust**                                                           |
|-------------------|------------------------------------|-------------------------------------------------------------------|
| **Performance**   | 2.861 ms                           | **667 ns**                                                        |
| **Memory Safety** | Limited Information in Backtrace   | Detailed Error Backtrace (Adjust RUST_BACKTRACE for more info)   |
| **Concurrency**   | 33.164 seconds                     | **1.121 seconds**                                                 |

### Performance

**Python**
```sh
Factorial of 20 is: 2,432,902,008,176,640,000
Time taken: 2.861 ms
```
**Rust**
```sh
Factorial of 20 is: 2,432,902,008,176,640,000
Time taken: 667 ns
```

### Memory Safety
- The Python program raises an IndexError with limited error information.
- The Rust program panics and provides a detailed error message.

### Concurrency
**Python**
```sh
Sum: 499,999,999,500,000,000
Time taken: 33.164 seconds
```

**Rust**
```sh
Sum: 499,999,999,500,000,000
Time taken: 1.121 seconds
```

Empower your AI projects with Rust! 🌐🤖 
