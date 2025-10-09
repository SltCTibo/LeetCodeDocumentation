fn find_min(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut middle = (low + high) / 2;
    let mut res = nums[middle];

    println!("START: {:?}", nums);

    while low <= high {

        println!("Low: {} - Middle: {} - High: {}", nums[low], nums[middle], nums[high]);

        if low == high {
            return nums[middle].min(res)
        }

        if nums[middle] < nums[high] {
            if middle == 0 { return nums[middle].min(res) }
            high = middle - 1;
            res = res.min(nums[middle]);
            middle = (low + high) / 2;
        } else if nums[middle] > nums[high] {
            if middle == nums.len() - 1 { return nums[middle].min(res) }
            low = middle + 1;
            res = res.min(nums[middle]);
            middle = (low + high) / 2;
        }
    }

    println!("OUT: Low: {} - Middle: {} - High: {} - Res: {}", nums[low], nums[middle], nums[high], res);

    res
}

fn main() {
    assert!(find_min(vec![3, 4, 5, 1, 2]) == 1);
    assert!(find_min(vec![4, 5, 6, 7, 0, 1, 2]) == 0);
    assert!(find_min(vec![11, 13, 15, 17]) == 11);
    assert!(find_min(vec![3,1,2]) == 1);
    assert!(find_min(vec![4,5,1,2,3]) == 1)
}
