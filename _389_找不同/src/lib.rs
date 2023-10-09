/*
 * @lc app=leetcode.cn id=389 lang=rust
 *
 * [389] 找不同
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
       
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(
            'e'.to_owned(),
            Solution::find_the_difference("abcd".to_owned(), "abcde".to_owned())
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            'y'.to_owned(),
            Solution::find_the_difference("".to_owned(), "y".to_owned())
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            'a'.to_owned(),
            Solution::find_the_difference("a".to_owned(), "aa".to_owned())
        );
    }
}

// 54/54 cases passed (4 ms)
// Your runtime beats 21.05 % of rust submissions
// Your memory usage beats 84.21 % of rust submissions (2.1 MB)
pub fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;

    let mut hm = HashMap::new();

    for c in s.chars() {
        if let Some(has) = hm.get_mut(&c) {
            *has += 1;
        } else {
            hm.insert(c, 1);
        }
    }

    for c in t.chars() {
        if let Some(has) = hm.get_mut(&c) {
            if *has - 1 == -1 {
                return c;
            } else {
                *has -= 1;
            }
        } else {
            return c;
        }
    }
    return char::MAX;
}
