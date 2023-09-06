/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */
struct Solution;
// @lc code=start
#[derive(PartialEq, Debug)]
enum DIR {
    Left,
    Right,
    None,
}
use core::num;
use std::{cmp::Ordering, collections::HashMap};
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if nums.len() < 3 {
            return res;
        }
        let mut nums = nums.clone();
        nums.sort();
        let mut r = nums.len() - 1;
        let mut l = 0;

        for i in 0..nums.len() {
            // 排完序后只用走走左半段来拿负数，总体是跑一半的
            if nums[i] > 0 {
                return res;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // 在数i和i-1遇到同样的，上一个数字已经做了同样的操作不用再计算
            }
            l = i + 1;
            r = nums.len() - 1;
            while l < r {
                if nums[i] + nums[l] + nums[r] == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    // 以下两个while都是为了在已经找到了类似的，将排布在一起的相同数字都过滤掉
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if nums[i] + nums[l] + nums[r] > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res
        // 用每个元素去想的话
    }
}

// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn test_one() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            crate::three_sum_0(nums),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 2, -2, -1];
        // -2 -1 1 2
        assert_eq!(crate::three_sum_0(nums), vec![] as Vec<Vec<i32>>);
    }
}

pub fn three_sum_0(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();
    let mut r = nums.len() - 1;
    let mut l = 0;
    let mut mid = (r + l) / 2;
    let mut res = vec![];
    let mut move_direction = DIR::None;
    let mut map = HashMap::new();
    // println!("测试： -4 -1 -1 0 1 2 ");
    // -4 -1 -1 0 1 2       ---        -1 -1 2
    //
    while l < mid && mid < r {
        // println!(
        //     "loop: [{}] \"{}\" [{}] \"{}\" [{}] \"{}\"",
        //     l, nums[l], mid, nums[mid], r, nums[r]
        // );

        match (nums[l] + nums[mid] + nums[r]).cmp(&0) {
            Ordering::Less => {
                if mid + 1 == r {
                    if move_direction == DIR::Left {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                    mid = (l + r) / 2;
                    // println!("reset: mid {} ", mid)
                } else {
                    mid += 1;
                    move_direction = DIR::Right;
                }
            }
            Ordering::Equal => {
                // println!(
                //     "add, {},{},{} , dir : {:?} ",
                //     nums[l], nums[mid], nums[r], move_direction
                // );
                let new_come_vec = vec![nums[l], nums[mid], nums[r]];
                let nums_str_vec: Vec<String> =
                    new_come_vec.iter().map(|&e| e.to_string()).collect();
                let nums_str = nums_str_vec.join(",");
                if let Some(&bl) = map.get(&nums_str) {
                    // println!("已经有了,{}", bl);
                } else {
                    map.insert(nums_str, true);
                    res.push(vec![nums[l], nums[mid], nums[r]]);
                }
                if move_direction == DIR::Right {
                    if mid + 1 == r {
                        if move_direction == DIR::Left {
                            r -= 1;
                        } else {
                            l += 1;
                        }
                        mid = (l + r) / 2;
                        // println!("reset: mid {} ", mid)
                    } else {
                        mid += 1;
                        move_direction = DIR::Right;
                    }
                } else {
                    if mid - 1 == l {
                        if move_direction == DIR::Left {
                            r -= 1;
                        } else {
                            l += 1;
                        }
                        mid = (l + r) / 2;
                        // println!("reset: mid {} ", mid)
                    } else {
                        mid -= 1;
                        move_direction = DIR::Left;
                    }
                }
            }
            Ordering::Greater => {
                if mid - 1 == l {
                    if move_direction == DIR::Left {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                    mid = (l + r) / 2;
                    // println!("reset: mid {} ", mid)
                } else {
                    mid -= 1;
                    move_direction = DIR::Left;
                }
            }
        }
    }
    res
}

// 在边界的控制配合如何退出和调换的时机太需要更高层的思考

// 反思：
// 这里的每一步的逻辑变换后的场景假设性没有做到位，
// 也就是自己容易忽略了每一步对于问题情境下的条件
// 无论如何还是把每一步的描述出来后，加以场景的设定条件判断