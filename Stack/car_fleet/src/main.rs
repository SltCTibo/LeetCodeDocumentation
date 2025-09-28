pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut vectors: std::collections::BTreeMap<usize, f32> = std::collections::BTreeMap::new();
    let mut last_vector: f32 = 0.0;

    let mut fleet: i32 = 0;

    for i in 0..position.len() {
        vectors.insert(
            position[i] as usize,
            (target - position[i]) as f32 / speed[i] as f32,
        );
    }

    for (_, vector) in vectors.iter().rev() {
        if last_vector < *vector {
            fleet += 1;
            last_vector = *vector;
        }
    }

    fleet
}

fn main() {
    assert!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]) == 3);
    assert!(car_fleet(10, vec![3], vec![3]) == 1);
    assert!(car_fleet(10, vec![6, 8], vec![3, 2]) == 2);
    assert!(car_fleet(10, vec![0, 2], vec![1, 1]) == 2);
    assert!(car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]) == 6);
}
