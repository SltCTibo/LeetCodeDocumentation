use std::{cmp::max, collections::BTreeSet};

fn longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let mut longest = 0;
    let sorted_nums: BTreeSet<i32> = nums.into_iter().collect();
    // let mut sorted_nums = nums;
    // sorted_nums.sort();

    let mut iter = sorted_nums.iter();
    let mut last_number: Option<i32> = None;
    let mut current_sequence = 1;

    while let Some(number) = iter.next() {
        if let Some(last) = last_number {
            if *number == last + 1 {
                current_sequence += 1;
            } else if *number == last {
                continue;
            } else {
                current_sequence = 1;
            }
        }

        last_number = Some(*number);
        longest = max(longest, current_sequence);
    }

    longest
}

fn main() {
    println!("Hello, world!");

    assert!(longest_consecutive_sequence(vec![100,4,200,1,3,2]) == 4);
    assert!(longest_consecutive_sequence(vec![0,3,7,2,5,8,4,6,0,1]) == 9);
    assert!(longest_consecutive_sequence(vec![1,0,1,2]) == 3);
    assert!(longest_consecutive_sequence(vec![1,0,1,2,10,11,12,13,14,15]) == 6);
    assert!(longest_consecutive_sequence(vec![]) == 0);
}
