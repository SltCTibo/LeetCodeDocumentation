use std::collections::HashMap;

pub fn best_one(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    nums.into_iter()
        .for_each(|num| *map.entry(num).or_insert(0) += 1);
    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.iter().take(k as usize).map(|x| x.0).collect()
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|c| *map.entry(*c).or_insert(0) += 1);

    let mut result: Vec<(i32, i32)> = map.iter().map(|(c, v)|  (*c, *v)).collect();
    result.sort_by_key(|(_, val)|*val);

    result.reverse();

    result.iter().enumerate().filter_map(|(v, (c, _))|if v < k as usize {Some(*c)} else { None}).collect::<Vec<i32>>()
}

fn main() {
    println!("Hello, world!");
    top_k_frequent(vec![1, 2, 2, 3, 3, 3, 3, 4], 1);
}
