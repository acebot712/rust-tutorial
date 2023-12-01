import time
import threading

def calculate_sum(start, end, result):
    local_sum = 0
    for i in range(start, end):
        local_sum += i
    result.append(local_sum)

def main():
    start_time = time.time()
    n = 1_000_000_000
    num_threads = 4  # You can adjust the number of threads

    chunk_size = n // num_threads
    result = []

    threads = []

    for _ in range(num_threads):
        start_range = chunk_size * _
        end_range = n if _ == num_threads - 1 else start_range + chunk_size

        thread_obj = threading.Thread(target=calculate_sum, args=(start_range, end_range, result))
        threads.append(thread_obj)
        thread_obj.start()

    for thread_obj in threads:
        thread_obj.join()

    total_sum = sum(result)

    end_time = time.time()

    print("Sum:", total_sum)
    print("Time taken:", end_time - start_time)

if __name__ == "__main__":
    main()
