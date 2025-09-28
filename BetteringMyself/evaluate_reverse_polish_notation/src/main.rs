fn evaluate_reverse_polish_notation(tokens: Vec<String>) -> i32 {

    let mut stack = Vec::<i32>::new();

    for t in tokens {
        if let Ok(nbr) = t.parse::<i32>() {
            stack.push(nbr);
        } else {
            let nbr2 = stack.pop().unwrap();
            let nbr1 = stack.pop().unwrap();

            match t.as_str() {
                "+" => {
                    stack.push(nbr1 + nbr2);
                },
                "-" => {
                    stack.push(nbr1 - nbr2);
                },
                "*" => {
                    stack.push(nbr1 * nbr2);
                },
                "/" => {
                    stack.push(nbr1 / nbr2);
                }
                _ => ()
            }
        }
    };

    stack[0]
}

fn main() {
    println!("Hello, world!");

    assert!(evaluate_reverse_polish_notation(vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()]) == 9);
    assert!(evaluate_reverse_polish_notation(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()]) == 6);
}
