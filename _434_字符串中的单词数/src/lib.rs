/*
 * @lc app=leetcode.cn id=434 lang=rust
 *
 * [434] 字符串中的单词数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let ss = s.split(' ');
        ss.filter(|x| { 
            *x != "" 
        }).collect::<Vec<&str>>().len() as i32
    }
}
// @lc code=end
