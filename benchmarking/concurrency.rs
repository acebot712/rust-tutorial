use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n: u64 = 1_000_000_000;
    let num_threads = 4; // You can adjust the number of threads

    let chunk_size = n / num_threads;
    let sum = Arc::new(Mutex::new(0u64));

    let mut handles = vec![];

    for thread_num in 0..num_threads {
        let sum = Arc::clone(&sum);
        let start_range = chunk_size * thread_num;
        let end_range = if thread_num == num_threads - 1 { n } else { start_range + chunk_size };

        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for i in start_range..end_range {
                local_sum += i;
            }
            let mut sum = sum.lock().unwrap();
            *sum += local_sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Sum: {}", *sum.lock().unwrap());
    println!("Time taken: {:?}", duration);
}
