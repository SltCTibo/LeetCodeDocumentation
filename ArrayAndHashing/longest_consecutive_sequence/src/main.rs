fn best_one(nums: Vec<i32>) -> i32 {
    let set: std::collections::HashSet<_> = nums.iter().collect();
    set.iter()
        .filter(|&&x| !set.contains(&(x - 1))) // ensure not part of a longer consecutive sequence
        .map(|&&x| (x..).take_while(|x| set.contains(x)).count())
        .max()
        .unwrap_or(0) as _
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut v = nums.clone();

    v.sort();

    for i in 0..v.len() {

        let mut sequence = 1;
        let mut precedent = v[i];
        'inner : for j in i+1..v.len() {
            if precedent + 1 != v[j] {
                if precedent == v[j] {
                    continue;
                }
                break 'inner;
            }
            precedent = v[j];
            sequence += 1;
        }

        if sequence > res {
            res = sequence;
        }

        if v.len() - i < res as usize {
            break;
        }
    }

    res
}

fn main() {
    let t = longest_consecutive(vec![100, 2, 200, 1, 3, 4]);
    println!("{t}");
    let t = longest_consecutive(vec![1, 2, 0, 1]);
    println!("{t}");
}
