class Solution {
public:
    constexpr inline std::string 
    gcdOfStrings(const std::string str1, const std::string str2) {
        if ((str1 + str2) != (str2 + str1)) { return ""; } 
        const size_t len1 = str1.size();
        const size_t len2 = str2.size();
        const size_t len_gcd = gcd(std::move(len1), std::move(len2));
        return str1.substr(0, len_gcd);
    }
};
