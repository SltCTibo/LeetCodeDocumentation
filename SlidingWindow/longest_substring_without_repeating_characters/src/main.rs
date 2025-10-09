fn length_of_longest_substring(s: String) -> i32 {
    let mut map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_index) = map.get(&c) {
            left = std::cmp::max(left, prev_index + 1);
        }
        map.insert(c, right);
        max_len = std::cmp::max(max_len, right - left + 1);
    }

    max_len as i32
}

fn main() {
    println!("Hello, world!");
    assert!(length_of_longest_substring("abcabcbb".to_string()) == 3);
    assert!(length_of_longest_substring("bbbbb".to_string()) == 1);
    assert!(length_of_longest_substring("pwwkew".to_string()) == 3);
}
