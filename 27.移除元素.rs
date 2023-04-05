/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if (nums.len() == 0) {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            if nums[right] == val {
                if right == 0 {
                    return 0;
                }
                right -= 1;
                continue;
            }
            if nums[left] == val {
                nums[left] = nums[right];
                right -= 1;
                continue;
            } else {
                left += 1;
            }
        }
        (right + 1) as i32
    }
}
// @lc code=end
