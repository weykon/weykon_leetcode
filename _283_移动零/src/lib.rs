/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>){
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
            j += 1;
        }
    }
}
// @lc code=end

// 快慢指针
// 快指针遇到非0就前置到慢指针位置，替换后慢指针跟上

// 新数组
// 使用 retain (保留) -> 返回true保留，false去掉
