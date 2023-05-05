use std::collections::HashMap;

fn main() {
    // 0, 1, 2, 5, 8
    let numbers = vec![0, 5, 8, 1, 2, 3];
    let median = median(numbers.clone());
    println!("Median for {:?} is {median}", numbers);

    let other_numbers = vec![2, 14, 28, 1, 0, 14, 1, 8, 14];
    let mode = mode(other_numbers.clone());
    println!("Mode for {:?} is {mode}", other_numbers);
}

fn median(numbers: Vec<i32>) -> f32 {
    let mut nums: Vec<i32> = numbers.clone();
    nums.sort();

    let numbers_count = nums.len();
    let middle = numbers_count / 2;
    if numbers_count % 2 == 0 {
        let left_mid = nums[middle - 1] as f32;
        let right_mid = nums[middle] as f32;
        (left_mid + right_mid) / 2.0
    } else {
        nums[middle] as f32
    }
}

fn mode(numbers: Vec<i32>) -> i32 {
    let nums_frequency = numbers.iter().fold(HashMap::new(), |mut acc, &num| {
        let num_entry = acc.entry(num).or_insert(0);
        *num_entry += 1;
        acc
    });

    let most_frequent_num = nums_frequency.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    *most_frequent_num.0
}
