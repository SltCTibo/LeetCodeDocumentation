fn encode(strs: Vec<String>) -> String {
    strs.join(" ")
}

fn decode(str: String) -> Vec<String> {
    str.split_whitespace().map(|c|c.to_string()).collect::<Vec<String>>()
}

fn main() {
    let strs: Vec<String> = vec!["thibault".to_string(), "barbe".to_string(), "le".to_string(), "plus".to_string(), "beau".to_string()];

    println!("{:?} = {:?}", strs, decode(encode(strs.clone())));
}
