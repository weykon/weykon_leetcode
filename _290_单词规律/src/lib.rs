/*
 * @lc app=leetcode.cn id=290 lang=rust
 *
 * [290] 单词规律
 */
struct Solution;
// @lc code=start
impl Solution {
    // 明明很简单的一道题，不清醒的时候什么都做不出来
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut a_map = HashMap::new();
        let mut b_map = HashMap::new();
        let pats: Vec<char> = pattern.chars().collect();
        let long_ss: Vec<&str> = s.split(" ").into_iter().collect();
        if pats.len() != long_ss.len() {
            return false;
        }
        for i in 0..pats.len() {
            if !a_map.contains_key(&pats[i]) && !b_map.contains_key(&long_ss[i]) {
                a_map.insert(pats[i], long_ss[i]);
                b_map.insert(long_ss[i], pats[i]);
            } else {
                if let Some(a) = a_map.get(&pats[i]) {
                    if *a != long_ss[i] {
                        return false;
                    }
                }
                if let Some(b) = b_map.get(&long_ss[i]) {
                    if *b != pats[i] {
                        return false;
                    }
                }
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
        assert_eq!(
            false,
            Solution::word_pattern("abba".to_owned(), "dog dog dog dog".to_owned())
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            true,
            Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned())
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            true,
            Solution::word_pattern("abc".to_owned(), "dog cat pig".to_owned())
        );
    }

    #[test]
    fn four() {
        assert_eq!(
            false,
            Solution::word_pattern("aba".to_owned(), "dog cat cat".to_owned())
        );
    }
}
