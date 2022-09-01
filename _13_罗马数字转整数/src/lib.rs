/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .zip(s.chars().skip(1).map(Some).chain(std::iter::once(None)))
            .map(|(first, second)| match (first, second) {
                ('I', Some('V') | Some('X')) => -1,
                ('X', Some('L') | Some('C')) => -10,
                ('C', Some('D') | Some('M')) => -100,

                ('I', _) => 1,
                ('V', _) => 5,
                ('X', _) => 10,
                ('L', _) => 50,
                ('C', _) => 100,
                ('D', _) => 500,
                ('M', _) => 1000,
                _ => unreachable!(),
            })
            .sum()
    }
}
// @lc code=end
