impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut out_chars: Vec<char> = Vec::new();
        let size1: usize = word1.len();
        let size2: usize = word2.len();
        let mut step: usize = 0;
        while step < std::cmp::min(size1, size2) {
            out_chars.push(word1.chars().nth(step).unwrap());
            out_chars.push(word2.chars().nth(step).unwrap());
            step += 1;
        }
        while step < std::cmp::max(size1, size2) {
            if step < size1 {
                out_chars.push(word1.chars().nth(step).unwrap());
            } else {
                out_chars.push(word2.chars().nth(step).unwrap());
            }
            step += 1;
        }
        return out_chars.into_iter().collect();
    }
}
