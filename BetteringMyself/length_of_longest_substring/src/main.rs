use std::{cmp::max, collections::HashMap};


fn longest_substring(s: String) -> i32 {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_index) = map.get(&c) {
            left = max(left, prev_index + 1);
        }
        map.insert(c, right);
        max_len = max(max_len, right - left + 1);
    }
    max_len as i32
}

fn main() {
    println!("Hello, world!");

    assert!(longest_substring("abcabcbb".to_string()) == 3);
    assert!(longest_substring("bbbbbb".to_string()) == 1);
    assert!(longest_substring("pwwkew".to_string()) == 3);
    assert!(longest_substring("aab".to_string()) == 2);
    assert!(longest_substring("abcdabcdeaaabbbeeeeeeccccddsdddddzzz".to_string()) == 5);
}
