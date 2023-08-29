/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 1;
        for fast in 1..nums.len() {
            if nums[fast] != nums[slow - 1] {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }
        slow as i32
    }
}
// @lc code=end

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    (slow + 1) as i32
}


// 1 1 1 2 2 3  ===>   1,1,1, same while of iter keep value on 1, then 1 != 2,  a ref back on first 1, slow add 1 to new postions
// 1 1 2
