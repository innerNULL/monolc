use std::cmp;


impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut left_offset: i32 = 0;
        let mut right_offset: i32 = 0;
        let mut out: i32 = i32::MAX;
        loop {
            let left_idx: i32 = start - left_offset;
            let right_idx: i32 = start + right_offset;
            let mut left_val: Option<i32> = None;
            let mut right_val: Option<i32> = None;
            if left_idx < 0 && right_idx >= nums.len() as i32 {
                break;
            }
            if left_idx >= 0 {
                left_val = Some(nums[left_idx as usize]);
                left_offset += 1;
            }
            if right_idx < nums.len() as i32 {
                right_val = Some(nums[right_idx as usize]);
                right_offset += 1;
            }
            match left_val {
                Some(x) => {
                    if x == target {
                        out = cmp::min(start - left_idx, out);
                    }
                },
                _ => {}
            }
            match right_val {
                Some(x) => {
                    if x == target {
                        out = cmp::min(right_idx - start, out);
                    }
                },
                _ => {}
            }
            if out != i32::MAX {
                break;
            }
        }
        return out;
    }
}
