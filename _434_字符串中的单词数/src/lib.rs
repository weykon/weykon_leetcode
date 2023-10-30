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

// 复盘题目的原则
// 1. 题目涉及到语言的特性应用
// 2. 新的思维要求
// 3. 数组数据操作和处理形式

// 

mod tests {
    use crate::Solution;
 
    #[test]
    fn one () { 
        assert_eq!(
            Solution::count_segments("Hello, my name is John".to_owned()),
            5 as i32
        )
    }
}