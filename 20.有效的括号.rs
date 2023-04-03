/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<&u8> = Vec::with_capacity(s.len());
        let s = s.as_bytes();
        for i in 0..s.len() {
            let char = &s[i];
            match char {
                b'{' => {
                    stack.push(&b'}');
                }
                b'[' => {
                    stack.push(&b']');
                }
                b'(' => {
                    stack.push(&b')');
                }
                _ => {
                    if stack.pop() != Some(char) {
                        return false;
                    }
                }
            };
        }
        return stack.is_empty();
    }
}
// @lc code=end
