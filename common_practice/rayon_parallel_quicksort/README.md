### Task: Parallel Quicksort using Rayon

**Goal:** Implement the quicksort algorithm to sort an array of integers in parallel using the Rayon crate.

### Starter Code:

Here's some code to help you get started:

`Cargo.toml`:

```toml
[dependencies]
rayon = "1.5.1"
```

`src/lib.rs`:

```rust
use rayon::prelude::*;

pub fn parallel_quicksort<T: Ord + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| parallel_quicksort(lo), || parallel_quicksort(hi));
}

fn partition<T: Ord>(v: &mut [T]) -> usize {
    // Implement the partitioning logic here
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_quicksort() {
        // Add tests here
    }
}
```

### Your Task:

Complete the `partition` function and write tests to verify that the `parallel_quicksort` function correctly sorts an array of integers.
