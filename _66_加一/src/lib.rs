/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                 digits[i] += 1;
                 return digits;
            }

            digits[i] = 0;

            if i == 0 {
                digits.insert(0, 1);
                return digits;
            }
        }

        digits
    }
}
// @lc code=end
