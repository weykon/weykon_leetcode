/*
 * @lc app=leetcode.cn id=412 lang=rust
 *
 * [412] Fizz Buzz
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 1..=n {
            match i {
                _ if i % 3 == 0 && i % 5 == 0 => {
                    res.push("FizzBuzz".to_owned());
                }
                _ if i % 3 == 0 => {
                    res.push("Fizz".to_owned());
                }
                _ if i % 5 == 0 => {
                    res.push("Buzz".to_owned());
                }
                _ => {
                    res.push(i.to_string());
                }
            }
        }
        res
    }
}
// @lc code=end
