/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        (2..=n).for_each(|i| {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        });
        dp[n as usize]
    }
}
// @lc code=end

// 这是首个我应对动态规划、斐波那契数列的思维熟悉
// 其中对于结果，作为一种状态来看待，比如这里的楼梯
// 走到第二层阶梯的结果作为一个状态，在这个状态下，
// 有走两次的一步，和走一次的两步
// 然后延伸到第三层，然后我们从这种状态的变换下
// 见规律后，构建递归算法

// 试一下原始的斐波那契
fn fibonacci(_in: u32) -> u32 {
    match _in {
        0 => 0,
        1 => 1,
        _ => fibonacci(_in - 1) + fibonacci(_in - 2),
    }
}
