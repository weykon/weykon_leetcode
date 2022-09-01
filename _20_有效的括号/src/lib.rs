/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut cursor_record = Vec::<char>::new();
        for char in s.chars() {
            if char == '(' || char == '[' || char == '{' {
                cursor_record.push(char);
            } else if match cursor_record.last() {
                Some('(') => char == ')',
                Some('[') => char == ']',
                Some('{') => char == '}',
                _ => false,
            } {
                cursor_record.pop();
            } else {
                return false;
            }
        }
        cursor_record.is_empty()
    }
}
// @lc code=end
