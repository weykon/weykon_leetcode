/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */
pub struct  Solution;
// @lc code=start
use std::collections::VecDeque ;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
	fn bfs(grid: &mut Vec<Vec<char>>, 
		ans: &mut i32, 
		row: usize, col: usize
	){
		let rows = grid.len() ;
		let cols = grid[0].len() ;
		grid[row][col] = '0' ;
		let mut vd: VecDeque<(usize, usize)> = VecDeque::new();
		vd.push_back((row, col));
		while !vd.is_empty() {
			for _ in 0..vd.len() {
				if let Some((x, y)) = vd.pop_front() {
					
					// right
					if y+1 < cols && grid[x][y+1] == '1' {
						vd.push_back((x, y+1));
						grid[x][y+1] = '0' ;
					}
					// down
					if x+1 < rows && grid[x+1][y] == '1' {
						vd.push_back((x+1, y));
						grid[x+1][y] = '0';
					}
					// left 
					if y > 0 && grid[x][y-1] == '1' {
						vd.push_back((x, y-1));
						grid[x][y-1] = '0';
					}
					// up 
					if x > 0 && grid[x-1][y] == '1' {
						vd.push_back((x-1, y));
						grid[x-1][y] = '0' ;
					}

				}
			}
		}
		*ans += 1;
	}
	for i in 0..grid.len() {
		for j in 0..grid[0].len(){
			if grid[i][j] == '1' {
				bfs(&mut grid, &mut ans, i, j);
			}
		}
	}
	ans
    }
}
// @lc code=end

