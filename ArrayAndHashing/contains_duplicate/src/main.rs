use std::collections::HashSet;

// Two Liner
pub fn best_one(nums: Vec<i32>) -> bool {
    let mut exists = HashSet::new();
    !nums.into_iter().all(|n| exists.insert(n))
}

// One Liner
pub fn best_one_bis(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut tmp = HashSet::new();
    for num in &nums {
        if !tmp.insert(num) {
            return true;
        }
    }
    return false;
}

fn main() {
    println!("Hello, world!");
}
