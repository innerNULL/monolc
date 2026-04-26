class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        if (nums.size() == 0) { return; }
        
        int32_t zero_count = 0;
        int32_t no_zero_idx = 0;
        for (auto x : nums) { zero_count += (x == 0); }
        for (int32_t i = 0; i < nums.size(); ++i) {
            //printf("dbg: no_zero_idx=%i\n", no_zero_idx);
            if (no_zero_idx < nums.size() - zero_count) {
                if (nums[i] != 0) {
                    nums[no_zero_idx] = nums[i];
                    no_zero_idx++; 
                }
            } else {
                break;
            }
        }   
        for (int i = nums.size() - zero_count; i < nums.size(); ++i) {
            nums[i] = 0;
        }
    }
};