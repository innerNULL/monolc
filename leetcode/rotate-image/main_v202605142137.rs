impl Solution {
    #[inline(always)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let dim: usize = matrix.len();
        for i in 0..dim {
            for j in 0..i {
                let tmp: i32 = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for i in 0..dim {
            let mut left: usize = 0;
            let mut right: usize = dim - 1;
            while left < right {
                let tmp: i32 = matrix[i][left];
                matrix[i][left] = matrix[i][right];
                matrix[i][right] = tmp;
                left += 1;
                right -= 1;
            }
        }
        return;   
    }
}
