using std::vector;
using std::string;


auto get_pal_flag(const std::string& s) -> vector<vector<bool>> {
    size_t n = s.size();
    vector<vector<bool>> out {n, vector<bool>(n, false)};
    for (int32_t i = n - 1; i >= 0; --i) {
        out[i][i] = true;
        for (int32_t j = i + 1; j < n; ++j) {
            out[i][j] = (
                s[i] == s[j] && (
                    j - i <= 1 || out[i + 1][j - 1]
                )
            );
        }
    }
    return out;
}


class Solution {
public:
    vector<vector<string>> partition(string s) {
        size_t n = s.size();
        vector<vector<bool>> is_pal = get_pal_flag(s);
        vector<vector<string>> out;
        vector<string> buffer;

        std::function<void(size_t)> dfs = [&] (size_t i) {
            if (i == n) {
                out.emplace_back(buffer);
                return;
            } else {
                for (size_t j = i; j < n; ++j) {
                    if (is_pal[i][j]) {
                        buffer.emplace_back(std::string_view(s).substr(i, j - i + 1));
                        dfs(j + 1);
                        buffer.pop_back();
                    }
                }
            }
        };
        dfs(0);
        return out;  
    }
};
