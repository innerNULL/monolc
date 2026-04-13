class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        word1_len: int = len(word1)
        word2_len: int = len(word2)
        if word1_len == 0 or word2_len == 0:
            return word1 + word2
        output: str = ""
        for i in range(max(word1_len, word2_len)):
            if i < word1_len and i < word2_len:
                output += word1[i]
                output += word2[i]
            elif i < word1_len:
                output += word1[i]
            else:
                output += word2[i]
        return output
        
