fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut high = matrix.len() - 1;
    let mut low = 0;

    while low <= high {
        let middle = (low + high) / 2;

        let mut inside_high = matrix[middle].len() - 1;
        let mut inside_low = 0;
        let mut inside_middle = inside_high;

        if target >= matrix[middle][inside_low] && target <= matrix[middle][inside_high] {
            
            while matrix[middle][inside_middle] != target {

                if inside_low >= inside_high { return false }

                if matrix[middle][inside_middle] > target {
                    if inside_middle == 0 { return false }
                    inside_high = inside_middle - 1;
                    inside_middle = (inside_low + inside_high) / 2;
                } else if matrix[middle][inside_middle] < target {
                    if inside_middle == matrix[middle].len() - 1 { return false }
                    inside_low = inside_middle + 1;
                    inside_middle = (inside_low + inside_high) / 2;
                }
            }

            return true;
        } else if target < matrix[middle][inside_low] {
            if middle == 0 { return false }
            high = middle - 1;
        } else if target > matrix[middle][inside_high] {
            if middle == matrix.len() -1 { return false }
            low = middle + 1;
        }
    }

    false
}

fn main() {
    assert!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3) == true);
    assert!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13) == false);
    assert!(search_matrix(vec![vec![1]], 1) == true);
    assert!(search_matrix(vec![vec![-9,-8,-6,-4,-4,-4,-3,-2,-1,0,0,2,4,5,7,9,9,10,10,12,13,13,14,15,16,16,18,19,21,21,23,24,26,28,29,29,31,31,33,34,35,35], vec![36,36,37,37,38,40,42,43,45,46,48,50,52,53,53,55,55,57,57,59,61,61,62,62,63,65,66,66,66,68,69,71,72,74,76,78,80,82,83,83,84,85], vec![86,87,89,89,90,92,94,95,97,99,100,101,101,103,103,104,104,106,108,109,110,111,112,113,113,114,114,116,118,119,120,122,122,123,125,126,128,130,130,132,134,136], vec![137,139,141,143,145,147,148,149,149,151,153,155,155,155,155,157,159,161,162,163,164,165,166,167,167,167,168,169,171,172,172,174,175,177,179,179,181,181,182,182,182,183]], 176) == false);
}
