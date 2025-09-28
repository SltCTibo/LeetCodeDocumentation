// FIRST IS ALWAYS (
// LAST IS ALWAYS  )

fn rec_fun(open: &mut Vec<char>, close: &mut Vec<char>, last: char, res: &mut Vec<String>, current: String) {

    if open.is_empty() && close.is_empty() {
        res.push(current);
        return
    }

    match last {
        '(' => {
            if !open.is_empty() {
                let mut new_open_current = current.clone();
                new_open_current.push('(');
                let mut new_open = open.clone();
                new_open.pop();
                rec_fun(&mut new_open, close, '(', res, new_open_current);
            }

            let mut new_close_current = current.clone();
            new_close_current.push(')');
            let mut new_close = close.clone();
            new_close.pop();
            rec_fun(open, &mut new_close, ')', res, new_close_current);
        }
        ')' => {
            if close.len() > open.len() {
                let mut new_close_current = current.clone();
                new_close_current.push(')');
                let mut new_close = close.clone();
                new_close.pop();
                rec_fun(open, &mut new_close, ')', res, new_close_current);
            }

            if !open.is_empty() {
                let mut new_open = open.clone();
                new_open.pop();
                let mut new_current = current.clone();
                new_current.push('(');
                rec_fun(&mut new_open, close, '(', res, new_current);
            }
        }
        _ => ()
    }
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];

    let mut open_stack = vec!['('; n as usize];
    let mut close_stack = vec![')'; n as usize];

    let last = open_stack.pop().unwrap();
    
    rec_fun(&mut open_stack, &mut close_stack, last, &mut res, "(".to_string());

    res
}

fn main() {
    println!("{:?}", generate_parenthesis(1));
    println!("{:?}", generate_parenthesis(2));
    println!("{:?}", generate_parenthesis(3));
}
