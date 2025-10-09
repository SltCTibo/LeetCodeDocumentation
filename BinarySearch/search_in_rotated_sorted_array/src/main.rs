fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;

    println!("START: {:?} - TARGET: {target}", nums);

    while low <= high {
        let middle = (low + high) / 2;

        if nums[middle] == target { return middle as i32 }

        if nums[low] <= nums[middle] {
            if target >= nums[low] && target < nums[middle] {
                if middle == 0 { return -1; }
                high = middle - 1;
            } else {
                if middle == nums.len() - 1 { return -1; }
                low = middle + 1;
            }
        } else if nums[middle] <= nums[high] {
            if target > nums[middle] && target <= nums[high] {
                if middle == nums.len() - 1 { return -1; }
                low = middle + 1;
            } else {
                if middle == 0 { return -1; }
                high = middle - 1;
            }
        }
    }

    -1
}

fn main() {
    assert!(search(vec![4,5,6,7,0,1,2], 0) == 4);
    assert!(search(vec![4,5,6,7,0,1,2], 3) == -1);
    assert!(search(vec![1], 0) == -1);
    assert!(search(vec![1], 1) == 0);
    assert!(search(vec![1,3], 3) == 1);
    assert!(search(vec![1,2,3,4,5,6], 4) == 3);
}
