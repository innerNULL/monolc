impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let non_zero_cnt: i32 = nums
            .iter()
            .map(|x| {
                if x.clone() == 0 {
                    0
                } else {
                    1
                }
            })
            .reduce(|acc, x| { acc + x })
            .unwrap();
        let mut write_ptr: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[write_ptr] = nums[i];
                write_ptr += 1;
            }
            if write_ptr as i32 >= non_zero_cnt {
                break;
            }
        }
        for i in (non_zero_cnt as usize)..nums.len() {
            nums[i] = 0;
        }
        return;
    }
}
