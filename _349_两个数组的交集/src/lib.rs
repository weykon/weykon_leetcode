/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::cmp;
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        let mut res = vec![];
        let (first, second) = if nums1.len() > nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        for i in first {
            set.insert(i);
        }
        for i in second {
            if let Some(v) = set.get(&i) {
                res.push(*v);
                set.remove(&i);
            }
        }
        res
    }
}
// @lc code=end
