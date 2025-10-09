// J'ai triché je ne trouvais pas pour optimiser
// Triche via indice de chatGPT

fn check_inclusion(s1: String, s2: String) -> bool {
    let mut to_compare: Vec<i32> = vec![0; 26];
    let mut current: Vec<i32> = vec![0; 26];
    let s = s2.chars().collect::<Vec<char>>();

    for char in s1.chars() {
        to_compare[char as usize - b'a' as usize] += 1;
    }

    for i in 0..s.len() {
        current[s[i] as usize - b'a' as usize] += 1;

        if i >= s1.len() - 1 {
            if to_compare.eq(&current) {
                return true;
            }

            current[s[i - (s1.len() - 1)] as usize - b'a' as usize] -= 1;
        }
    }

    false
}

fn main() {
    println!("Hello, world!");

    assert_eq!(check_inclusion("ab".to_string(), "eidoooba".to_string()), true);
    println!("");
    assert_eq!(check_inclusion("ab".to_string(), "eidboaooo".to_string()), false);
    println!("");
    assert_eq!(check_inclusion("adc".to_string(), "dcda".to_string()), true);
}
