/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
fn get_vall_num(s: &String, start: usize, end: usize) -> i32 {
    match s.get(start..end) {
        Some(sub_str) => {
            return get_val(sub_str);
        }
        None => panic!("error"),
    }
}

fn get_val(s: &str) -> i32 {
    match s {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        _ => 0,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let end: usize = s.len();
        let mut i: usize = 0;
        let mut num: i32 = 0;
        while i < end {
            if i == end - 1 {
                num += get_vall_num(&s, i, i + 1);
                i += 1;
            } else {
                let n1 = get_vall_num(&s, i, i + 1);
                let n2 = get_vall_num(&s, i + 1, i + 2);
                if n1 < n2 {
                    num += n2 - n1;
                    i += 2;
                } else {
                    num += n1;
                    i += 1;
                }
            }
        }
        num
    }
}
// @lc code=end
