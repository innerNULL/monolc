class Solution {
public:
    bool isSubsequence(string s, string t) {
        if (s.size() > t.size()) { return false; }
        if (s.size() == 0) { return true; }
        
        int32_t s_idx = 0;
        int32_t t_idx = 0;
        while (t_idx < t.size() && s_idx < s.size()) {
            if (t.at(t_idx) == s.at(s_idx)) {
                s_idx++;
            } 
            t_idx++;
        }
        if (s_idx == s.size()) { 
            return true;
        } else {
            return false;
        }
    }
};