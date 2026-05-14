impl Solution {
    #[inline(always)]
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        let mut out: bool = true;
        let n: i32 = nums.len() as i32 - 1;
        nums.sort();
        for i in 0..(n as usize + 1) {
            if i < n as usize {
                out = out & (nums[i] == i as i32 + 1);
            } else {
                out = out & (nums[i] == n);
            }
        }
        out
    }
}
