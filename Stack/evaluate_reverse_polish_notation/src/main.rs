
pub fn best_one(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    fn is_operator(s: &str) -> bool {
        s == "+" || s == "-" || s == "*" || s == "/"
    }

    for token in tokens {
        if is_operator(&token) {
            let ele2 = stack.pop().unwrap();
            let ele1 = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => ele1 + ele2,
                "-" => ele1 - ele2,
                "*" => ele1 * ele2,
                "/" => ele1 / ele2, // Note: Division can panic for zero division
                _ => 0,
            };
            stack.push(result);
        } else {
            let num: i32 = token.parse().unwrap();
            stack.push(num);
        }
    }

    stack.pop().unwrap()
}


fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut idx = 2;
    let mut tmp: Vec<String> = tokens.clone();

    loop {
        if tmp.len() == 1 {
            return tmp[0].parse::<i32>().unwrap()
        }

        match tmp[idx].as_str() {
            "+" => {
                let res = tmp[idx - 2].parse::<i32>().unwrap() + tmp[idx - 1].parse::<i32>().unwrap();
                tmp[idx - 2] = res.to_string();
                idx = idx - 2;
                tmp.remove(idx + 1);
                tmp.remove(idx + 1);
            }
            "-" => {
                let res = tmp[idx - 2].parse::<i32>().unwrap() - tmp[idx - 1].parse::<i32>().unwrap();
                tmp[idx - 2] = res.to_string();
                idx = idx - 2;
                tmp.remove(idx + 1);
                tmp.remove(idx + 1);
            }
            "/" => {
                let res = tmp[idx - 2].parse::<i32>().unwrap() / tmp[idx - 1].parse::<i32>().unwrap();
                tmp[idx - 2] = res.to_string();
                idx = idx - 2;
                tmp.remove(idx + 1);
                tmp.remove(idx + 1);
            }
            "*" => {
                let res = tmp[idx - 2].parse::<i32>().unwrap() * tmp[idx - 1].parse::<i32>().unwrap();
                tmp[idx - 2] = res.to_string();
                idx = idx - 2;
                tmp.remove(idx + 1);
                tmp.remove(idx + 1);
            }
            _ => {
                idx = idx + 1;
            }
        }
    }
}

fn main() {
    println!("{}", eval_rpn(vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"].into_iter().map(|s| s.to_owned()).collect()));
}
