/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        // my mem go
        let mut res = vec![0; nums.len()];
        let (mut l, mut r, mut set) = (0usize, nums.len(), nums.len());

        // [-7,-3,-2,0,1,2,3]
        // [ 0, 0, 0,0,0,0,0]
        while l < r {
            set -= 1;
            if nums[l].abs() < nums[r - 1].abs() {
                r -= 1;
                res[set] = nums[r] * nums[r];
            } else {
                res[set] = nums[l] * nums[l];
                l += 1;
            }
        }
        res
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
    }
    #[test]
    fn two() {
        assert_eq!(vec![1], Solution::sorted_squares(vec![-1]));
    }

    #[test]
    fn three() {
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
    }
}
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    // wild
    for i in 0..nums.len() {
        nums[i] = nums[i] * nums[i];
    }
    nums.sort();
    nums
}

// 基本全靠内存来
pub fn sorted_squares_two(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    let (mut i, mut j, mut pos) = (0usize, nums.len(), nums.len());
    while i < j {
        pos -= 1; // -7, -3, 2, 3, 11
        if nums[i].abs() > nums[j - 1].abs() {
            ans[pos] = nums[i] * nums[i];
            i += 1;
        } else {
            j -= 1;
            ans[pos] = nums[j] * nums[j];
        }
    }
    ans
}
