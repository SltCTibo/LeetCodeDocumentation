use std::collections::{BTreeMap, HashMap};

pub fn best_one(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs.iter() {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key = chars.into_iter().collect::<String>();

        map.entry(key).or_insert(Vec::new()).push(s.clone());
    }

    map.into_values().collect()
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<BTreeMap<char, i32>, Vec<String>> = HashMap::new();

    for str in strs.iter() {
        let mut key = BTreeMap::new();
        str.chars().for_each(|c| *key.entry(c).or_insert(0) += 1);

        map.entry(key).or_insert(Vec::new()).push(str.clone());
    }

    map.into_values().collect()
}

fn main() {
    let test1: Vec<String> = Vec::from([
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string()]);
    let test2: Vec<String> = Vec::from([
        "".to_string()]);
    let test3: Vec<String> = Vec::from([
        "a".to_string()]);

    println!("Test 1: {:?}", group_anagrams(test1));
    println!("Test 2: {:?}", group_anagrams(test2));
    println!("Test 3: {:?}", group_anagrams(test3));
}
