use std::sync::Arc;
use std::thread;

pub fn sum_vector(v: &[i32]) -> i32 {
    let v = Arc::new(v.to_vec());
    let mut num_threads = num_cpus::get();

    if num_threads > v.len() {
        num_threads = v.len() / 10;
    }
    let chunk_size = v.len() / num_threads;

    // let mut handles: Vec<thread::JoinHandle<i32>> = vec![];

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let chunk = v.clone();

            thread::spawn(move || {
                chunk[i * chunk_size..(i + 1) * chunk_size]
                    .iter()
                    .sum::<i32>()
            })
        })
        .collect();

    let mut total_sum = 0;

    for handle in handles {
        total_sum += handle.join().unwrap();
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let total_sum = sum_vector(&v);
        println!("The total sum is {}", total_sum);
        assert_eq!(total_sum, v.into_iter().sum());
    }
}
