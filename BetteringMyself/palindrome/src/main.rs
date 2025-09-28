fn is_palindrome(s: String) -> bool {
    let cleaned: Vec<char> = s.chars().filter_map(|c|
        if c.is_alphanumeric() {
            Some(c.to_lowercase().nth(0).unwrap())
        } else {
            None
        }
    ).collect::<Vec<char>>();

    let mut reversed: Vec<char> = cleaned.clone();
    reversed.reverse();

    cleaned == reversed
}

fn main() {
    println!("Hello, world!");

    assert!(is_palindrome("kayak".to_string()));
    assert!(is_palindrome("thibault".to_string()) == false);
}
