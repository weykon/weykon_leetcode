/*
 * @lc app=leetcode.cn id=561 lang=rust
 *
 * [561] 数组拆分
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let chunk2_nums = nums.chunks(2);
        chunk2_nums.fold(0, |acc, x| acc + std::cmp::min(x[0], x[1]))
    }
}
// @lc code=end
