
use std::error::Error;
mod base_one;

/*
 * @lc app=leetcode.cn id=463 lang=rust
 *
 * [463] 岛屿的周长
 */
pub struct Solution;
use base_one::here_my_first_version_for_this_problem;

// @lc code=start
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        
    }
}

struct Pos {
    w_id: usize,
    h_id: usize,
}

// @lc code=end
mod tests {
    use crate::Solution;

    #[test]
    fn test_my_fn() {
        let my_test: Vec<Vec<i32>> = vec![
            [0, 1, 0, 0].to_vec(),
            [1, 1, 1, 0].to_vec(),
            [0, 1, 0, 0].to_vec(),
            [1, 1, 0, 0].to_vec(),
        ];
        assert_eq!(16, Solution::island_perimeter(my_test))
    }
}

