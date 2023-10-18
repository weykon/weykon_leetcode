use core::num;

/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut pre = nums[0];
        let mut ret = vec![];

        nums.windows(2).for_each(|win| {
            if win[1] != win[0] + 1 {
                if win[0] == pre {
                    ret.push(format!("{}", pre));
                } else {
                    ret.push(format!("{}->{}", pre, win[0]));
                }
                pre = win[1];
            }
        });

        if let Some(&last) = nums.last() {
            if last == pre {
                ret.push(format!("{}", pre));
            } else {
                ret.push(format!("{}->{}", pre, last));
            }
        }
        ret
    }
}
// @lc code=end
