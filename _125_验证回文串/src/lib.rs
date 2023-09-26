/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a != b {
                return false;
            }
        }

        true
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false);
    }

    #[test]
    fn two() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
    }
}
