use std::collections::HashMap;

fn best_one(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1
            } else {
                res.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }

    return res;
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in nums.iter() {
        *map.entry(*num).or_insert(0) += 1;
    }

    for (key, nb) in map.iter() {
        for (k, v) in map.iter() {
            if key == k && *v == 1 {
                continue;
            }

            let to_find = (*key + *k) * -1;

            if (to_find == *k && *v == 1) || (to_find == *key && *nb == 1) || (to_find == *key && to_find == *k && *nb < 3) {
                continue;
            }
            if map.contains_key(&to_find) {
                let mut new_vec = vec![*key, *k, to_find];
                new_vec.sort();
                if !result.contains(&new_vec) {
                    result.push(new_vec);
                }
            }
        }

    }

    result
}

fn main() {
    let res = three_sum(vec![-1,0,1,2,-1,-4]);
    let res2 = three_sum(vec![0,0,0]);
    println!("Result : {:?}", res);
    println!("Result : {:?}", res2);
}
