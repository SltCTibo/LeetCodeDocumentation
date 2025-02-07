fn trap(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut i = 0;

    if let Some(max) = height.iter().max() {
        if let Some((mid_x, middle)) = height.iter().enumerate().find(|(u, &v)|  v == *max) {
            'outer: while i < mid_x {
                if height[i] > 0 {
                    let mut j = i + 1;
                    while j < height.len() {
                        if height[j] >= height[i] {
                            for x in i..j {
                                result += height[i] - height[x];
                            }
                            i = j;
                            continue 'outer;
                        }
                        j += 1;
                    }
                }
                i += 1;
            }

            i = height.len() - 1;

            'outer: while i > mid_x {
                if height[i] > 0 {
                    let mut j = i - 1;
                    while j >= mid_x {
                        if height[j] >= height[i] {
                            for x in j+1..i {
                                result += height[i] - height[x];
                            }
                            i = j;
                            continue 'outer;
                        }
                        j -= 1;
                    }
                }
                i -= 1;
            }
        }
    }

    result as i32
}

fn main() {
    println!("Res: {}", trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
    println!("Res: {}", trap(vec![4,2,0,3,2,5]));
    println!("Res: {}", trap(vec![4,2,3]));
    println!("Res: {}", trap(vec![2,0,2]));
}
