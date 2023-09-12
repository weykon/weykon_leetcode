use std::{result, vec};

/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 1,3 2,6 8,10 15,18
        // 1-2-3-6! 8-10|15-18
        let mut intervals = intervals;
        // 1,3 2,6 5,8 6,9
        // 1-9

        // 1,3 4,5 6,7 2,9
        // 1-9
        intervals.sort();
        // [[1, 3], [2, 9], [4, 5], [6, 7]] this is breaking changing ; 
        println!("intervals.sort( ); {:?} ", intervals);
        let mut res: Vec<Vec<i32>> = vec![];
        let mut cur = vec![intervals[0][0], intervals[0][1]];
        for interval in intervals {
            if interval[0] > cur[1] {
                res.push(cur);
                cur = interval;
            } else if interval[1] > cur[1] {
                cur[1] = interval[1];
            }
        }
        res.push(cur);

        res
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![4, 5], vec![6, 7], vec![2, 9]]),
            [[1, 9]]
        );
    }
}
