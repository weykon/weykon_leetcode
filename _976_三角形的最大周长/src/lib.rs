/*
 * @lc app=leetcode.cn id=976 lang=rust
 *
 * [976] 三角形的最大周长
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let nums_iter = nums.iter().rev();
        let mut mark = 0;
        let mut len_triangle = 0;
        while mark >= 0 && mark < (nums.len() - 2) {
            let a = nums[mark];
            let b = nums[mark - 1];
            let c = nums[mark - 2];
            if a + b > c && a + c > b && b + c > a {
                len_triangle = std::cmp::max(len_triangle, a + b + c);
            } else if a + b <= c {
                mark += 1;
            }
        }
        len_triangle
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(12, Solution::largest_perimeter(vec![3,4,5]))
    }
}
