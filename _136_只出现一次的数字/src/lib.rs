/*
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * [136] 只出现一次的数字
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0; 

        for i in 0..nums.len() {
            ans ^= nums[i] ;
        }

        ans
    }
}
// @lc code=end

