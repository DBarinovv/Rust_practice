### Exercise: Using Threads to Calculate the Sum of a Vector

You will create a function that takes a vector of integers and calculates the sum of its elements using multiple threads.

1. **Split the Vector:** Divide the vector into equal parts based on the number of available CPU cores.
2. **Create Threads:** For each part of the vector, create a thread that calculates the sum of that part.
3. **Join Threads:** Wait for all the threads to complete and collect the results.
4. **Sum Results:** Sum up the results from all the threads to get the final sum of the vector.

Here's a skeleton to help you get started:

```rust
use std::thread;

fn sum_vector(v: &[i32]) -> i32 {
    let num_threads = num_cpus::get(); // you can use the `num_cpus` crate to get the number of CPU cores
    let chunk_size = v.len() / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        // Split the vector into chunks
        let chunk = &v[i * chunk_size..(i + 1) * chunk_size];

        // Spawn a thread to sum the chunk
        let handle = thread::spawn(move || {
            chunk.iter().sum()
        });

        handles.push(handle);
    }

    let mut total_sum = 0;

    for handle in handles {
        // Join the thread and add its result to the total sum
        total_sum += handle.join().unwrap();
    }

    total_sum
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let total_sum = sum_vector(&v);
    println!("The total sum is {}", total_sum);
}
```

Note that this code assumes that the vector's length is evenly divisible by the number of CPU cores. In a real-world scenario, you'd need to handle the remainder as well.

If you want to run this code, you will need to add the `num_cpus` crate to your `Cargo.toml`:

```toml
[dependencies]
num_cpus = "1.13.0"
```
