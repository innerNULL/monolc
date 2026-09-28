fn get_pal_flag(s: &String) -> Vec<Vec<bool>> {
    let b: &[u8] = s.as_bytes();
    let n: usize = s.len();
    let mut out: Vec<Vec<bool>> = vec![ vec![false; n]; n ];
    (0..n).rev().for_each(
        |i| {
            out[i][i] = true;
            (i + 1..n).for_each(
                |j| {
                    out[i][j] = {
                        (b[i] == b[j]) && (j - i <= 1 || out[i + 1][j - 1])
                    };
                }
            )
        }
    );
    out 
}


impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n: usize = s.len();
        let pal_flag: Vec<Vec<bool>> = get_pal_flag(&s);

        let mut out: Vec<Vec<String>> = Vec::new();
        let mut buffer: Vec<String> = Vec::new();

        fn dfs(
            out: &mut Vec<Vec<String>>, 
            buffer: &mut Vec<String>,
            i: usize, 
            n: usize,
            s: &String,
            pal_flag: &Vec<Vec<bool>>
        ) {
            if i == n {
                out.push(buffer.clone());
                return;
            } else {
                for j in i..n {
                    if pal_flag[i][j] {
                        buffer.push(s[i..j + 1].to_string());
                        dfs(out, buffer, j + 1, n, s, pal_flag);
                        buffer.pop();
                    }
                }
            }
        };
        dfs(&mut out, &mut buffer, 0, n, &s, &pal_flag);
        out
    }
}
