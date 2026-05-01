fn mark_area(
    x: i32, 
    y: i32, 
    grid: &mut Vec<Vec<char>>
) {
    if (
        x < 0 || y < 0 
        || x >= grid.len() as i32 || y >= grid[0].len() as i32
    ) {
        return;
    }
    if grid[x as usize][y as usize] == '0' {
        return;
    }
    grid[x as usize][y as usize] = '0';
    mark_area(x - 1, y, grid);
    mark_area(x, y - 1, grid);
    mark_area(x + 1, y, grid);
    mark_area(x, y + 1, grid);
    return;
}


impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let dim0: usize = grid.len();
        let dim1: usize = grid[0].len();
        let mut cnt: i32 = 0;
        for i in 0..dim0 {
            for j in 0..dim1 {
                let val: char = grid[i][j];
                if val == '1' {
                    cnt += 1;
                    mark_area(i as i32, j as i32, &mut grid);
                }
            }
        }
        return cnt;
    }
}
