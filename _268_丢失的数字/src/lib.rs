/*
 * @lc app=leetcode.cn id=268 lang=rust
 *
 * [268] 丢失的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.push(-1);
        let mut i = 0;
        let mut mark = 0;
        while i <= nums.len() {
            
        }
        i as i32
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }

    #[test]
    fn two() {
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
    }
}

pub fn missing_number_basis(nums: Vec<i32>) -> i32 {
    // 先提交一版简单的
    let mut nums = nums;
    nums.sort();
    let mut i = 0;
    loop {
        if let Some(&num) = nums.get(i as usize) {
            if num != i {
                break;
            } else {
                i += 1;
            }
        } else {
            break;
        }
    }
    i
}

pub fn missing_number_2(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut set: HashSet<i32> = HashSet::new();

    nums.iter().for_each(|&x| {
        set.insert(x);
    });

    for i in 0..=nums.len() {
        if let Some(_) = set.get(&(i as i32)) {
            continue;
        } else {
            return i as i32;
        }
    }
    return 0;
}
