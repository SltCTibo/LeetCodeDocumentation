use std::{collections::HashMap};

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self { map: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key)
            .or_insert_with(|| Vec::new())
            .push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(inner) = self.map.get(&key) {
            let mut high = inner.len() - 1;
            let mut low = 0;
            let mut middle = high;

            println!("SEARCH: {timestamp} - IN: {:?}", inner);

            while inner[middle].0 != timestamp {

                println!("LOW: {low} - MID: {middle} - HIGH: {high}");

                if low == high {
                    if inner[middle].0 > timestamp {
                        return String::new()
                    } else {
                        return inner[middle].1.clone()
                    }
                } else if low == high - 1 {
                    if inner[high].0 > timestamp && inner[low].0 < timestamp {
                        return inner[low].1.clone()
                    } else if inner[high].0 < timestamp {
                        return inner[high].1.clone()
                    }
                }
                
                if inner[middle].0 > timestamp {
                    if middle == 0 {
                        if inner[middle].0 > timestamp {
                            return String::new()
                        } else {
                            return inner[middle].1.clone()
                        }
                    }
                    high = middle - 1;
                    middle = (low + high) / 2;
                } else if inner[middle].0 < timestamp {
                    if middle == inner.len() - 1 {
                        if inner[middle].0 > timestamp {
                            return String::new()
                        } else {
                            return inner[middle].1.clone()
                        }
                    }
                    low = middle + 1;
                    middle = (low + high) / 2;
                }
            }

            return inner[middle].1.clone()
        }

        String::new()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {
    let mut map = TimeMap::new();
    map.set("foo".to_string(), "bar".to_string(), 5);
    map.set("foo".to_string(), "bar2".to_string(), 10);
    map.set("foo".to_string(), "bar3".to_string(), 14);
    println!("Get: {}", map.get("foo".to_string(), 4));
    println!("Get: {}", map.get("foo".to_string(), 6));
    println!("Get: {}", map.get("foo".to_string(), 11));
}
