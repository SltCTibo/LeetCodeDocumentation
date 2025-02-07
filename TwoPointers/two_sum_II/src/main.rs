use std::cmp::Ordering;

fn best_one(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len()-1;

    while i < j {
        match (numbers[i]+numbers[j]).cmp(&target) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => return vec![(i+1) as i32, (j+1) as i32],
        }
    }
    unreachable!();
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, number) in numbers.iter().enumerate() {
        if *number > target / 2 {
            break;
        }
        let to_search = target - *number;

        if let Some((j, _)) = numbers.iter().enumerate().find(|(idx, &x)| *idx != i && x == to_search) {
            return vec![i as i32 + 1, j as i32 + 1];
        }
    }

    vec![]
}

fn main() {
    let result = two_sum(vec![1, 2, 3], 4);

    println!("Res: {:?}", result);

}
