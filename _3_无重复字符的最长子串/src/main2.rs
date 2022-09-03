/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        let mut sub_str_start: usize = 0;
        let mut longest = 0;
        let mut map: HashMap<char, usize> = HashMap::new();

        for (idx, c) in s.char_indices() {
            map.entry(c)
                .and_modify(|old_idx| {
                    if *old_idx >= sub_str_start {
                        // got a repetition
                        longest = max(longest, idx - sub_str_start);
                        sub_str_start = *old_idx + 1;
                    }
                    *old_idx = idx;
                })
                .or_insert(idx);
        }

        (max(longest, s.len() - sub_str_start) as u32).try_into().unwrap()
    }
}
// @lc code=end

// "abcabcbb"
// "bbbbb"
// "pwwkew"
// "wppcaspcads"
// "wpwaapadcjiurt"
