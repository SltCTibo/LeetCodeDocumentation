

fn min_window(s: String, t: String) -> String {
    use std::str::FromStr;
    let mut to_compare: Vec<i32> = vec![0; 173];
    let mut current: Vec<i32> = vec![0; 173];

    let target = t.chars().collect::<Vec<char>>();
    let s1 = s.chars().collect::<Vec<char>>();

    let mut to_find = t.len();

    let mut left = 0;
    let mut best_left = 0;
    let mut best_right = 0;

    for char in target {
        to_compare[char as usize] += 1;
    }

    for i in 0..s1.len() {
        if to_compare[s1[i] as usize] > 0 {
            current[s1[i] as usize] += 1;
            if current[s1[i] as usize] <= to_compare[s1[i] as usize] {
                to_find -= 1;
            }
        }

        while to_find == 0 {
            if i - left + 1 < best_right - best_left + 1 || best_right == 0 {
                best_right = i;
                best_left = left;
                if i - left + 1 == t.len() {
                    println!("Answer IN: {:?}", String::from_str(&s[left..i+1]).expect("Good string"));
                    return String::from_str(&s[left..i+1]).expect("Good string");
                }
            }

            if to_compare[s1[left] as usize] > 0 {
                current[s1[left] as usize] -= 1;
                if current[s1[left] as usize] < to_compare[s1[left] as usize] {
                    to_find += 1;
                }
            }

            left += 1
        }
    }

    if best_left >= best_right {
        String::new()
    } else {
        println!("Answer OUT: {:?}", String::from_str(&s[best_left..best_right+1]).expect("Good string"));
        String::from_str(&s[best_left..best_right+1]).expect("Good string")
    }
}

fn main() {
    println!("Hello, world!");

    assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
    assert_eq!(min_window("a".to_string(), "a".to_string()), "a".to_string());
    assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
    assert_eq!(min_window("bbaa".to_string(), "aba".to_string()), "baa".to_string());
    assert_eq!(min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()), "cwae".to_string());
}
