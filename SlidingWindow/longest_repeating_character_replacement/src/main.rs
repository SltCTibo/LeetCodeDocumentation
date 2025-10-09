fn character_replacement(s: String, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut left = 0;
    let mut max_len = 0;
    let mut window: HashMap<char, i32> = HashMap::new();
    let ch: Vec<char> = s.chars().collect();

    for right in 0..s.len() {

        *window.entry(ch[right]).or_insert(0) += 1;

        if (right - left + 1) as i32 - window.values().max().unwrap() > k {
            window.entry(ch[left]).and_modify(|v| if *v > 1 { *v -= 1 } else { *v = 0 });
            left += 1;
        }

        max_len = max_len.max(right - left + 1);
    }

    max_len.max(s.len() - left) as i32
}

fn main() {
    println!("Hello, world!");

    assert!(character_replacement("ABBB".to_string(), 2) == 4);
    assert!(character_replacement("AAB".to_string(), 1) == 3);
    assert!(character_replacement("ABAB".to_string(), 2) == 4);
    assert!(character_replacement("AZERTYUIOP".to_string(), 2) == 3);
    assert!(character_replacement("AABABBA".to_string(), 1) == 4);
}
/* 
// Tester la plus grande window et déplacer le curseur vers la droite
// Si pas possible
*/