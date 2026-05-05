fn mark(
    grid: &Vec<Vec<char>>,
    recorder: &mut Vec<Vec<i8>>, 
    row: usize, 
    col: usize,
) {
    if row >= recorder.len() 
        || col >= recorder[0].len() 
        || (row as i32) < 0 
        || (col as i32) < 0
        || recorder[row][col] != -1
    {
        return;    
    }
    if grid[row][col] == '1' {
        recorder[row][col] = 1;
        mark(grid, recorder, row - 1, col);
        mark(grid, recorder, row, col - 1);
        mark(grid, recorder, row + 1, col);
        mark(grid, recorder, row, col + 1);
    } else {
        recorder[row][col] = 0;
    }
    return;
}


impl Solution {
    pub fn num_islands(
        mut grid: Vec<Vec<char>>
    ) -> i32 {
        let row_dim: usize = (&grid).len();
        let col_dim: usize = (&grid)[0].len();
        let mut recorder: Vec<Vec<i8>> = vec![
            vec![-1; col_dim]; row_dim
        ];
        let mut cnt: i32 = 0;
        for i in 0..row_dim {
            for j in 0..col_dim {
                if (&recorder)[i][j] != -1 {
                    continue;
                } else if (&grid)[i][j] == '1' {
                    cnt += 1;
                    mark(&grid, &mut recorder, i, j);
                }
            }
        }
        return cnt;
    }
}
