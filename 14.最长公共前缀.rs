/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        } else if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut i = 0;
        let mut is_end = false;
        while !is_end {
            for j in 0..strs.len() {
                if strs[j].len() <= i {
                    is_end = true;
                    break;
                }
                if j == 0 {
                    continue;
                }
                if strs[j][i..i + 1] != strs[0][i..i + 1] {
                    is_end = true;
                    break;
                }
            }
            i += 1;
        }
        return strs[0][0..i - 1].to_string();
    }
}
// @lc code=end
