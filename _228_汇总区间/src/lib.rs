use core::num;

/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        
    }
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }
    // 开头直接放第一个
    let mut pre = nums[0];
    let mut ret = vec![];

    nums.windows(2).for_each(|win| {
        // 后一个数和前一个数的值对比，不相等才处理
        if win[1] != win[0] + 1 {
            // 如果当前的第一数跟之前设置的数相同
            if win[0] == pre {
                ret.push(format!("{}", pre));
            } else {
                ret.push(format!("{}->{}", pre, win[0]));
            }
            pre = win[1];
        }
    });


    if let Some(&last) = nums.last() {
        if last == pre {
            ret.push(format!("{}", pre));
        } else {
            ret.push(format!("{}->{}", pre, last));
        }
    }
    ret
}

// @lc code=end


// 找到了
// 前后顺序状态性质的题目
// 这里看到的直接上windows

// 不相等才去处理，可能很难明白，按照我们的惯性，
// 0 1 2 OK 然后去构建数据输出，但是这里的处理不是
// 继续看看，他是在下面做
// 这里很反直觉，他是偏偏大胆地在遇到相同的不去想，
// 只关注在到了不一样的地步，就去补充前面的间段，
// 因为能作为完成的，前面的间段是完好的
// 有一种“要好的还是坏的？我先把坏的全要，剩下的都是好的”的意思