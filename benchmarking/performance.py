import time

def factorial(n):
    if n == 0:
        return 1
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result

start_time = time.time()
n = 20  # You can change this to a larger number
result = factorial(n)
end_time = time.time()

print("Factorial of", n, "is:", result)
print("Time taken:", end_time - start_time)
