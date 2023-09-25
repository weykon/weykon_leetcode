/*
 * @lc app=leetcode.cn id=1046 lang=rust
 *
 * [1046] 最后一块石头的重量
 */
struct Solution;
// 74/74 cases passed (0 ms)
// Your runtime beats 100 % of rust submissions
// Your memory usage beats 12.5 % of rust submissions (2.2 MB)
// @lc code=start
impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        stones.sort();
        let mut i = stones.len() - 1;
        while i > 0 {
            if stones[i] > stones[i - 1] {
                let new_num = (stones[i] - stones[i - 1]).clone();
                stones.pop();
                stones.pop();
                stones.push(new_num);
                stones.sort();
                i -= 1;
            } else {
                stones.pop();
                stones.pop();
                if let Some(_) = i.checked_sub(2) {
                    i -= 2;
                } else {
                    i = 0;
                }
            }
        }

        if let Some(res) = stones.pop() {
            return res;
        } else {
            return 0;
        }
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn two() {
        assert_eq!(Solution::last_stone_weight(vec![1, 3]), 2);
    }

    #[test]
    fn three() {
        assert_eq!(Solution::last_stone_weight(vec![2, 2]), 0);
    }
}
