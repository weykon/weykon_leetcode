/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|&b| b == ' ')
            .take_while(|&b| b != ' ')
            .count() as i32
    }
}
// @lc code=end
