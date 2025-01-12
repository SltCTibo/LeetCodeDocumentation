pub fn best_one(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);

    map.into_values().all(|v| v == 0)
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut tmp = s.clone();
    for c in t.chars() {
        if tmp.contains(c) {
            let u = tmp.find(c).unwrap();
            tmp.remove(u);
        } else {
            return false
        }
    }

    tmp.len() == 0
}

fn main() {
    println!("Hello, world!");
}
