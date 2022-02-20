// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful
// here) of the list.

use std::collections::HashMap;

fn mean(v: Vec<i32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let sum: i32 = v.iter().sum();
    Some(sum as f32 / v.len() as f32)
}

fn median(v: &mut Vec<i32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }

    v.sort();
    let mid = v.len() / 2;

    if v.len() % 2 == 1 {
        return Some(v[mid] as f32);
    }

    mean(vec![v[mid], v[mid - 1]])
}

fn mode(v: &mut Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut map = HashMap::new();
    for key in v {
        let count = map.entry(key).or_insert(0);
        *count += 1;
    }

    let mut max_key = 0;
    let mut max_value = 0;

    for (key, value) in map {
        if value > max_value {
            max_value = value;
            max_key = *key;
        }
    }

    Some(max_key)
}

fn test_median(mut v: &mut Vec<i32>) {
    match median(&mut v) {
        Some(i) => println!("Numbers: {:?} : Median: {}", v, i),
        None => println!("None"),
    }
}

fn test_mean(v: &mut Vec<i32>) {
    match mean(v.to_vec()) {
        Some(i) => println!("Numbers: {:?} : Mean: {}", v, i),
        None => println!("None"),
    }
}

fn test_mode(v: &mut Vec<i32>) {
    match mode(v) {
        Some(i) => println!("Numbers: {:?} : Mode: {}", v, i),
        None => println!("None"),
    }
}

fn main() {
    //Test cases:

    // odd number of elements
    let mut test = vec![1, 9, 2, 8, 3, 7, 4, 6, 5];
    test_median(&mut test);

    // Even number of elements
    test = vec![-1, 1, 2, -2];
    test_median(&mut test);

    // No elements
    test.clear();
    test_median(&mut test);

    // Test mean
    test = vec![1, 2];
    test_mean(&mut test);

    test = vec![];
    test_mean(&mut test);

    test = vec![1, 1, 1, 1, 2, 2, 3, 4, 5, 5, 5, 5, 5];
    test_mode(&mut test);

    test = vec![];
    test_mode(&mut test);

    test = vec![1, 2, 3, 4, 5];
    test_mode(&mut test);
}
