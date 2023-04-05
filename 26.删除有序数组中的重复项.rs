/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let len = nums.len();
        while j < len {
            if nums[i] == nums[j] {
                j += 1;
            } else {
                if i + 1 != j {
                    i += 1;
                    nums[i] = nums[j];
                } else {
                    i += 1;
                    j += 1;
                }
            }
        }
        //nums.dedup();
        //nums.len() as i32
        (i + 1) as i32
    }
}
// @lc code=end
