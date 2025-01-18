fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];

    let ve = nums
        .iter()
        .filter_map(|&v| if v != 1 { Some(v) } else { None })
        .collect::<Vec<i32>>();

    for (i, num) in nums.iter().enumerate() {
        if *num == 1 || *num == -1 {
            let nbr: i32 = ve.iter().product();
            vec.push(nbr * *num);
        } else {
            vec.push(
                nums.iter()
                    .enumerate()
                    .filter_map(|(j, &n)| if j != i && n != 1 { Some(n) } else { None })
                    .collect::<Vec<i32>>()
                    .iter()
                    .product(),
            );
        }
    }
    vec
}

fn main() {
    println!("Hello, world!");
}
