/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        let half = str.len() / 2;
        let left = str.bytes().take(half);
        let right = str.bytes().rev().take(half);

        left.eq(right)
    }
}

// TODO: it's too slow. find a new solution
// @lc code=end
