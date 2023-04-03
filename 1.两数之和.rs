/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(k) => return vec![*k, i as i32],
                None => {
                    map.insert(nums[i], i as i32);
                }
            }
        }
        panic!("not found");
    }
}
// @lc code=end
