/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 || (x < 10 && x > 0) {
            return true;
        } else if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut num = x;
        let mut half = 0;
        while num > half {
            half = half * 10 + num % 10;
            num /= 10;
        }
        if num == half || half / 10 == num {
            return true;
        }
        false
    }
}
// @lc code=end
