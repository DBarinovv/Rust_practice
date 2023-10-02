use std::collections::{HashMap, VecDeque};

/// Finds and returns the first unstable number in the given sequence of `nums` using the specified `window_size`
///
/// # Arguments
/// * `nums` - A vector of i32 representing the sequence of numbers
/// * `window_size` - The size of the window to check for instability
///
/// # Returns
/// * Returns the first unstable number in the sequence
/// * Returns -1 if no unstable number is found
pub fn find_unstable_number(nums: Vec<i32>, window_size: usize) -> i32 {
    let mut window: VecDeque<i32> = VecDeque::with_capacity(window_size);
    let mut window_map: HashMap<i32, i32> = HashMap::with_capacity(window_size);

    // Initialize the window and window_set with the first `window_size` elements
    for &num in nums.iter().take(window_size) {
        window.push_back(num);
        *window_map.entry(num).or_insert(0) += 1;
    }

    // let mut line_cnt = window_size;
    for &num in nums.iter().skip(window_size) {
        // line_cnt += 1;
        let mut found = false;

        // Check whether there exist two distinct numbers in the window that sum up to num
        for &val in window.iter() {
            let complement = num - val;
            if let Some(&count) = window_map.get(&complement) {
                if complement != val || count > 1 {
                    found = true;
                    break;
                }
            }
        }

        // If no such pair is found, return num
        if !found {
            // println!("Line = {line_cnt}");
            return num;
        }

        // Update the window and window_map for the next iteration
        if let Some(removed) = window.pop_front() {
            if let Some(count) = window_map.get_mut(&removed) {
                *count -= 1;
                if *count == 0 {
                    window_map.remove(&removed);
                }
            }
        }

        window.push_back(num);
        *window_map.entry(num).or_insert(0) += 1;
    }

    // Return -1 if all numbers in the sequence follow the rule
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_5() {
        let nums = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let window_size = 5;
        println!("{}", find_unstable_number(nums, window_size)); // Output: 127
    }

    use std::error;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    #[test]
    fn window_100_from_file() -> Result<(), Box<dyn error::Error>> {
        let path = Path::new("src/input.txt");
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let nums: Vec<i32> = reader
            .lines()
            .filter_map(|elem| elem.ok().and_then(|line| line.parse().ok()))
            .collect();

        let window_size = 200;
        println!("{}", find_unstable_number(nums, window_size));
        Ok(())
    }
}
