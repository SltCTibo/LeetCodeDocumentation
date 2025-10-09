fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut output: Vec<i32> = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    let mut left = 0;

    for right in 0..nums.len() {

        while let Some(idx) = deque.back() {
            if nums[right] < nums[*idx] {
                break;
            } else {
                deque.pop_back();
            }
        }

        deque.push_back(right);

        if right - left + 1 >= k as usize {

            output.push(nums[*deque.front().unwrap()]);

            if left == *deque.front().unwrap() {
                deque.pop_front();
            }

            left += 1;
        }
    }

    output
}

fn main() {
    println!("Hello, world!");
    assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    assert_eq!(max_sliding_window(vec![9,10,9,-7,-4,-8,2,-6], 5), vec![10,10,9,2]);
}
