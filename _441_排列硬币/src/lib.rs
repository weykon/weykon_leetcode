/*
 * @lc app=leetcode.cn id=441 lang=rust
 *
 * [441] 排列硬币
 */

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((8.0 * n as f64 + 1.0).sqrt() / 2.0 - 0.5) as i32
    }
}
// @lc code=end
