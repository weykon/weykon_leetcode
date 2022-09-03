/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
pub struct Solution;
mod main2;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut positions = [0; 128];
        let (mut max_len, mut left) = (0, 1);

        for (b, i) in s.bytes().zip(1..) {
            let right = &mut positions[(b - b' ') as usize];
            match *right {
                0 => *right = i,
                _ => {
                    max_len = max_len.max(i - left);
                    left = left.max (*right + 1);
                    *right = i;
                }   
            }
        }

        max_len.max(s.len() as i32 - left + 1 )
    }
}
// @lc code=end

// "abcabcbb"
// "bbbbb"
// "pwwkew"
// "wppcaspcads"
// "wpwaapadcjiurt"
// "waduppaskjd"
