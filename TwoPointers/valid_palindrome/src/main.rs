fn valid_palindrome(s: String) -> bool {
    let mut t: Vec<char> = s
        .chars()
        .filter_map(|c|
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            })
        .collect::<Vec<char>>();

    let b = t.clone();
    t.reverse();

    b == t
}

fn main() {
    println!("Hello, world!");
}
