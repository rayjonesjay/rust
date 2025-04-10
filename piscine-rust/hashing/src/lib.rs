use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut mean = 0.0;
    let len = list.len();
    for i in 0..len {
        mean += list[i]as f64;
    }
    mean/len as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec(); // clone the slice into a new Vec
    sorted.sort(); // sort the vector in place

    let len = sorted.len();
    if len == 0 {
        panic!("Cannot compute median of an empty list");
    }

    if len % 2 == 1 {
        sorted[len / 2] // middle value
    } else {
        let mid = len / 2;
        (sorted[mid - 1] + sorted[mid]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();

    // Count frequency of each number
    for &num in list {
        *count.entry(num).or_insert(0) += 1;
    }

    // Find the number with the highest frequency
    let mut max_count = 0;
    let mut mode = list[0]; // default to first number

    for (&number, &freq) in count.iter() {
        if freq > max_count {
            max_count = freq;
            mode = number;
        }
    }

    mode
}