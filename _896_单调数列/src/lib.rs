use core::num;

/*
 * @lc app=leetcode.cn id=896 lang=rust
 *
 * [896] 单调数列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }
        let mut positive_order = 0;
        let mut move_cursor = 0;
        for i in 0..nums.len() {
            if nums[move_cursor] != nums[i] {
                if nums[move_cursor] > nums[i] {
                    positive_order = -1;
                } else {
                    positive_order = 1;
                }
                move_cursor = i;
                break;
            } else {
                continue;
            }
        }
        if positive_order == 0 {
            return true;
        } else {
            for i in move_cursor..(nums.len() - 1) {
                if positive_order == 1 {
                    if nums[i] <= nums[i + 1] {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    if nums[i] >= nums[i + 1] {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
