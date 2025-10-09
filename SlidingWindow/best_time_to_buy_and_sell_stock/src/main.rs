use std::i32;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lowest = i32::MAX;

    /* window start */
    for price in prices {
        lowest = lowest.min(price);
        max_profit = max_profit.max(price - lowest);
        println!("max_profit: {max_profit}");
    }

    max_profit
}

fn main() {
    println!("Hello, world!");

    assert!(max_profit(vec![7,1,5,3,6,4]) == 5);
    assert!(max_profit(vec![7,6,4,3,1]) == 0);
    println!("");
    assert!(max_profit(vec![1,2]) == 1);
}
