
fn is_valid(s: String) -> bool {
    let mut opened = String::new();

    for ch in s.chars() {
        if ch == '(' || ch == '[' || ch == '{' {
            opened += &String::from(ch);
        } else {
            if let Some(last_char) = opened.pop() {
                if last_char as u8 + 1 != ch as u8 && last_char as u8 + 2 != ch as u8 {
                    return false;
                }
            } else {
                return false;
            }
        } 
    }

    return opened.len() == 0;
}

fn main() {
    println!("{:?}", is_valid("(())".to_string()));
    println!("{:?}", is_valid("((())".to_string()));
}
