/*
 * @lc app=leetcode.cn id=485 lang=rust
 *
 * [485] 最大连续 1 的个数
 */
struct Solution;
// @lc code=start
impl Solution {
    // 输入：nums = [1,1,0,1,1,1]
    // 输出：3
    // 输入：nums = [1,0,1,1,0,1]
    // 输出：2
    // 输入：nums = [0,0,1,1,0,1]
    // 输出：2
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut max = 0;

        while p2 < nums.len() {
            if nums[p2] == 1 {
                if p1 <= p2 {
                    max = std::cmp::max(p2 - p1 + 1, max);
                } else {
                    p1 = p2;
                }
            }else { 
                p1 = p2+1;
            }
            p2 += 1;
        }

        max as i32
    }
}
// @lc code=end
