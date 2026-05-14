unsafe fn run(
    mat: *mut Vec<Vec<i32>>,
) {
    let mut start: usize = 0;
    let mut end: usize = (*mat).len() - 1;
    while start < end {
        // shouldn't be `=end`
        for i in 0..(end - start) {
            //println!("(dbg) start={}, end={}, i={}", start, end, i);
            // cache
            let up_val: i32 = (*mat)[start][start + i];
            let right_val: i32 = (*mat)[start + i][end];
            let bottom_val: i32 = (*mat)[end][end - i];
            let left_val: i32 = (*mat)[end - i][start];
            // update
            (*mat)[start + i][end] = up_val;
            (*mat)[end][end - i] = right_val;
            (*mat)[end - i][start] = bottom_val;
            (*mat)[start][start + i] = left_val;
        }
        start += 1;
        end -= 1;
    }
    return;
}


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // Using `unsafe` to make it faster
        unsafe { run(matrix as *mut Vec<Vec<i32>>); }
        return; 
    }
}
