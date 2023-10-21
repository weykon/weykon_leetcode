/*
 * @lc app=leetcode.cn id=414 lang=rust
 *
 * [414] 第三大的数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        let (mut a, mut b, mut c) = (i64::MIN, i64::MIN, i64::MIN);
        for n in nums {
            let n = n as i64;
            if n > a {
                c = b;
                b = a;
                a = n;
            } else if n < a && n > b {
                c = b;
                b = n;
            } else if n < b && n > c {
                c = n;
            }
        }
        return if c == i64::MIN { a as i32 } else { c as i32 };
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
    }

    #[test]
    fn two() {
        assert_eq!(Solution::third_max(vec![1, 1, 2]), 2);
    }
}
