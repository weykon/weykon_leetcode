/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        } else if t.len() == 0 {
            return false;
        }
        let mut a = 0;
        let mut b = 0;
        let s_max = s.len() - 1;
        let b_len = t.len() - 1;
        let ss: Vec<char> = s.chars().collect();
        let ts: Vec<char> = t.chars().collect();

        while a <= s_max {
            let char_a = ss[a];
            let char_b = ts[b];
            if char_a == char_b {
                a += 1;
                if b + 1 > b_len {
                    break;
                } else {
                    b += 1;
                }
            } else {
                if b + 1 > b_len {
                    return false;
                } else {
                    b += 1;
                }
            }
        }
        if a != s_max + 1 {
            return false;
        } else {
            return true;
        }
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(
            Solution::is_subsequence("acb".to_owned(), "ahbgdc".to_owned()),
            false
        );
    }
}
