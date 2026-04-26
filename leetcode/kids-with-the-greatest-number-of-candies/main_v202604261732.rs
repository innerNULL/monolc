impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_cnt: i32 = candies.iter().max().unwrap().clone();
        return candies
          .iter()
          .map(
            |x| {
                if x + extra_candies >= max_cnt {
                    true
                } else {
                    false
                }
            }
          )
          .collect::<Vec<bool>>();
    }
}
