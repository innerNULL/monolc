impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_sub: Vec<char> = Vec::new();
        let mut t_idx: usize = 0;
        let mut s_idx: usize = 0;
        while s_idx < s.len() && t_idx < t.len() {
            //println!("s_idx={}, t_idx={}", s_idx, t_idx);
            if s.chars().nth(s_idx) == t.chars().nth(t_idx) {
                t_sub.push(s.chars().nth(s_idx).unwrap().clone());
                s_idx += 1;
            } 
            t_idx += 1;
        }
        //println!("{:?}", t_sub);
        return t_sub.into_iter().collect::<String>() == s;
    }
}
