/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let phase: HashMap<char, usize> = HashMap::new();

        let mut re_run_index:usize = 0;
        let steps = s.chars().take(re_run_index).enumerate();
        let all = s.chars().enumerate();

        loop {
            match all.next(){
                None => {break;}
                Some(x) => {
                    match steps.next() {
                        None => {all.next();}
                        Some(y) => {
                            match phase.get(&y.1) {
                                Some(i) => {
                                    re_run_index = y.0 + 1;
                                    phase.insert(y.1, y.0);  
                                    all.next();
                                },
                                _ => {
                                    phase.insert(y.1, y.0);  
                                },
                            }
                        }
                    }
                   
                }  
            }
           phase.len();
        }
    }
}
// @lc code=end

// "abcabcbb"
// "bbbbb"
// "pwwkew"
// "wppcaspcads"
// "wpwaapadcjiurt"
