/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */
struct Solution;
// @lc code=start
impl Solution {
    //     74/74 cases passed (12 ms)
    // Your runtime beats 100 % of rust submissions
    // Your memory usage beats 53.52 % of rust submissions (3.7 MB)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();

        for i in nums {
            if let Some(_) = set.get(&i) {
                return true;
            } else {
                set.insert(i);
            }
        }
        false
    }
}
// @lc code=end

// the simple runtime 91.44% - memory 90.14%
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort();
    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            return true;
        }
    }
    false
}
