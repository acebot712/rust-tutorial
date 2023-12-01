def main():
    nums = []

    # Add 10 million integers to the list
    for i in range(1, 10_000_001):
        nums.append(i)

    # Attempt to access an out-of-bounds element
    value = nums[10_000_001]  # This will raise an IndexError

    print("This line won't be reached due to IndexError")

if __name__ == "__main__":
    main()
