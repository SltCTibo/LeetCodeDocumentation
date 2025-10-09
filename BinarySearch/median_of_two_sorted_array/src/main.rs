use std::cmp::{max, min};

fn triche_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums = [nums1, nums2].concat();

    nums.sort();

    println!("{:?}", nums);

    if nums.len() % 2 == 0 {
        (nums[nums.len() / 2 - 1] as f64 + nums[nums.len() / 2] as f64) / 2.0
    } else {
        nums[nums.len() / 2] as f64
    }
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

    let high: f64 = *max(nums1.iter().max().unwrap(), nums2.iter().max().unwrap()) as f64;
    let low: f64 = *min(nums1.iter().min().unwrap(), nums2.iter().min().unwrap()) as f64;

    (low + high) / 2.0
}

fn main() {
    assert!(find_median_sorted_arrays(vec![1,3], vec![2]) == 2.0);
    assert!(find_median_sorted_arrays(vec![1,2], vec![3,4]) == 2.5);
    assert!(find_median_sorted_arrays(vec![1,3,5,8], vec![2,4,6,7]) == 4.5);
}
