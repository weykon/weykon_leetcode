/*
 * @lc app=leetcode.cn id=1528 lang=rust
 *
 * [1528] 重新排列字符串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {

        let letters: Vec<char> = s.chars().collect();
        let mut res = vec!['a'; s.len()];
        for (idx, &item) in indices.iter().enumerate() {
            if letters[idx] == res[item as usize]{
                continue;
            }
            res[item as usize] = letters[idx];
        }

        res.iter().collect::<String>()
    }
}
// @lc code=end

