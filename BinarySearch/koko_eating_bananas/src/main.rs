fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut high = *piles.iter().max().unwrap();
    let mut low = 1;
    let mut ans = high;
    let mut found = false;

    while low <= high {
        let middle = (low + high) / 2;
        let mut hours_spent: i64 = 0;

        for pile in &piles {
            hours_spent += (pile + middle - 1) as i64 / middle as i64;
        }

        if hours_spent > h as i64 {
            low = middle + 1;
        } else if hours_spent <= h as i64 {
            println!("Trouvé");
            ans = middle;
            high = middle - 1;
        }
    }

    println!("Answer: {ans}");

    ans
}

fn main() {
    assert!(min_eating_speed(vec![3, 6, 7, 11], 8) == 4);
    assert!(min_eating_speed(vec![30, 11, 24, 4, 20], 5) == 30);
    assert!(min_eating_speed(vec![30,11,23,4,20], 6) == 23);
    assert!(min_eating_speed(vec![312884470], 312884469) == 2);
    assert!(min_eating_speed(vec![332484035,524908576,855865114,632922376,222257295,690155293,112677673,679580077,337406589,290818316,877337160,901728858,679284947,688210097,692137887,718203285,629455728,941802184], 823855818) == 14);

    assert!(min_eating_speed(vec![312884470], 968709470) == 1);
    assert!(min_eating_speed(vec![805306368,805306368,805306368], 1000000000) == 3)
}

