fn search_rust(nums: Vec<i32>, target: i32) -> i32 {
    if let Some((n, _)) = nums.iter().enumerate().find(|(_, x) | **x == target) {
        return n as i32;
    }

    -1
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut high = nums.len() - 1;
    let mut low: usize = 0;
    let mut middle = high;

    println!("START: {:?}, TARGET: {target}", nums);

    while nums[middle] != target {
        if low == high { return -1 }

        if nums[middle] > target {
            if middle == 0 { return -1; }
            high = middle - 1;
            middle = (low + high) / 2;
        } else if nums[middle] < target {
            if middle == nums.len() - 1 { return -1; }
            low = middle + 1;
            middle = (low + high) / 2;
        }
    }

    middle as i32
}

fn main() {
    assert!(search(vec![-1,0,3,5,9,12], 9) == 4);
    assert!(search(vec![-1,0,3,5,9,12], 2) == -1);
    assert!(search(vec![5], -5) == -1);
    assert!(search(vec![2, 5], 0) == -1);
    assert!(search(vec![-1, 0, 5], -1) == 0);

}
