use std::vec;

/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 * [860] 柠檬水找零
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        // 5,5,5,10,20
        let (mut five, mut ten) = (0, 0);
        for v in bills.iter() {
            match v {
                5 => {
                    five += 1;
                }
                10 => {
                    five -= 1;
                    ten += 1;
                }
                _ => {
                    if ten != 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        five -= 3;
                    }
                }
            }

            if five < 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end
