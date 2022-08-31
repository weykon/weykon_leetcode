/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            match hm.get(&num) {
                Some(&index) => {
                    return vec![index, i as i32];
                }
                _ => {hm.insert(target - num, i as i32);}
            }
        }
        vec![]
    }
}

// @lc code=end
