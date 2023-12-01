# Rust Tutorial
This repository consists of solutions I wrote while problem solving to get better at Rust language.

## Why learn Rust?

🤖 AI Engineers, boost your skills with Rust! 🦀

⚙️ Speed: Rust's efficiency enhances AI performance.
🛡️ Safety: Memory safety guards your models.
🚀 Concurrency: Perfect for parallel AI tasks.

📚 Learn Rust:
- [Official Rust Book](https://doc.rust-lang.org/book/)

## Benchmarking against Python

Table Summary:-

|                   |             **Python**            |                              **Rust**                             |
|:-----------------:|:---------------------------------:|:-----------------------------------------------------------------:|
| **Performance**   | 2861.02294921875ns                | **667ns**                                                             |
| **Memory Safety** | Limited Information in  Backtrace | Adjust RUST_BACKTRACE values to get more detailed error backtrace |
| **Concurrency**   | 33.16351079940796s                | **1.120522166s**                                                      |
### Performance
**Python**
```sh
Factorial of 20 is: 2432902008176640000
Time taken: 2861.02294921875ns
```

**Rust**
```sh
Factorial of 20 is: 2432902008176640000
Time taken: 667ns
```

### Memory Safety
The Python program will raise an IndexError, but it won't provide as detailed information about the error's origin as Rust does.

The Rust program will panic and provide a clear error message indicating the issue and where it occurred.

### Concurrency
**Python**
```sh
Sum: 499999999500000000
Time taken: 33.16351079940796s
```
**Rust**
```sh
Sum: 499999999500000000
Time taken: 1.120522166s
```



Empower your AI projects with Rust! 🌐🤖 #AIEngineering #RustLang 🦀💻
