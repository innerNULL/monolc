

void mark_island(
        std::vector<std::vector<char> >& recorder, 
        const vector<vector<char>>& grid, 
        int32_t start_row, int32_t start_col) {
    int32_t row_num = grid.size();
    int32_t col_num = grid[0].size();
    
    if ( (start_row >= row_num || start_row < 0) 
            || (start_col >= col_num || start_col < 0) ) {
        return; 
    } 
    if (recorder[start_row][start_col] != '1') { return; }   
    
    //printf("dbg: start_row=%i, start_col=%i\n", start_row, start_col);
    recorder[start_row][start_col] = '2';
    mark_island(recorder, grid, start_row, start_col + 1);
    mark_island(recorder, grid, start_row + 1, start_col);
    mark_island(recorder, grid, start_row, start_col - 1);
    mark_island(recorder, grid, start_row - 1, start_col);
}


class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int32_t count = 0;
        std::vector<std::vector<char> > recorder = grid;
        
        for (int32_t i = 0;  i < grid.size(); ++i) {
            for (int32_t j = 0; j < grid[0].size(); ++j) {
                //printf("dbg: i=%i, j=%i\n", i, j);
                if (recorder[i][j] == '1') {
                    //printf("dbg: i=%i, j=%i\n", i, j);
                    count += 1;
                    mark_island(recorder, grid, i, j);
                }
            }
        }
        return count;
    }
};