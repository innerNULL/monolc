impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut val1: i32 = nums1[idx1];
        let mut val2: i32 = nums2[idx2];
        let mut max_dist: i32 = 0;
        while idx1 < nums1.len() && idx2 < nums2.len() {
            val1 = nums1[idx1];
            val2 = nums2[idx2];
            //println!("Current idxs: {}, {}", idx1, idx2);
            //println!("Current vals: {}, {}", val1, val2);
            if idx1 > idx2 {
                idx2 += 1;
            } else if val1 > val2 {
                idx1 += 1;
            } else {
                max_dist = std::cmp::max(max_dist, (idx2 - idx1) as i32);
                idx2 += 1;
                //println!("One valid pair!");
            }
        }
        return max_dist;
    }
}
