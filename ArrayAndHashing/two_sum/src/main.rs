pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index: usize = 0;
    let mut index2: usize = 0;

    while index < nums.len() {
        index2 = index + 1;
        while index2 < nums.len() {
            if nums[index] + nums[index2] == target {
                let mut vec: Vec<i32> = Vec::<i32>::new();
                vec.push(index as i32);
                vec.push(index2 as i32);
                return vec
            }
            index2 += 1;
        }
        index += 1;
    }
    return Vec::<i32>::new()
}

fn main() {
    println!("Hello, world!");
}
