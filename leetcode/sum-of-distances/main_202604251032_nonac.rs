use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut out: Vec<i64> = vec![0 as i64; nums.len()];
        let mut ivt_idx: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            let val: i32 = nums[i];
            match ivt_idx.get_mut(&val) {
                Some(idxs) => {
                    idxs.push(i);
                },
                None => {
                    ivt_idx.insert(val, vec![i]);
                }
            }
        }
        for (val, idxs) in ivt_idx {
            for i in 0..idxs.len() {
                let idx: usize = idxs[i];
                if i == 0 {
                    for j in 1..idxs.len() {
                        let other_idx: usize = idxs[j];
                        out[idx] += (other_idx as i64 - idx as i64).abs();
                    }
                } else if i == idxs.len() - 1 {
                    for j in 0..i {
                        let other_idx: usize = idxs[j];
                        out[idx] += (other_idx as i64 - idx as i64).abs();
                    }
                } else {
                    for j in 0..i {
                        let other_idx: usize = idxs[j];
                        out[idx] += (other_idx as i64 - idx as i64).abs();
                    }
                    for j in (i + 1)..idxs.len() {
                        let other_idx: usize = idxs[j];
                        out[idx] += (other_idx as i64 - idx as i64).abs();
                    }
                }
            }
        }
        return out;
    }
}
