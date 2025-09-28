fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack: Vec<(usize, &i32)> = vec![];

    let mut iter = temperatures.iter().enumerate();
    let mut value = iter.next();

    while let Some(temp) = value {
        if stack.len() > 0 {
            let mut last_value = stack.last();
            'inner: while let Some(last) = last_value {
                if temp.1 > &last.1 {
                    res[last.0] = temp.0 - last.0;
                } else {
                    break 'inner;
                }
                stack.pop();
                last_value = stack.last();
            }
        }
        stack.push(temp);
        value = iter.next();
    }

    res.iter().map(|v| *v as i32).collect::<Vec<i32>>()
}

fn main() {
    println!("{:?}", daily_temperatures(vec![73,74,75,71,69,72,76,73]))
}
