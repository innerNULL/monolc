class Solution {
public:
    string mergeAlternately(string word1, string word2) {
        if (word1 == "" || word2 == "") { return word1 + word2; }
        int32_t word1_len = word1.size();
        int32_t word2_len = word2.size();
        std::string output = "";
        for (int32_t i = 0; i <= std::max(word1_len, word2_len); ++i) {
            if (i < word1_len && i < word2_len) {
                output += word1[i];
                output += word2[i];
            } else if (i < word1_len) {
                output += word1[i];
            } else {
                output += word2[i];
            }
        }
        return output.c_str();
    }
};
