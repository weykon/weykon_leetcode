/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut z = Vec::new();
        nums.retain(|x| {
         if *x != 0 { true } else { z.push(0); false }
        });
        &nums.append(&mut z);
    }
}
// @lc code=end


// 快慢指针

// 新数组