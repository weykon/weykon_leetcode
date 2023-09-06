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
use std::{cmp::Ordering, collections::HashMap};
impl Solution {}

// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn test_one() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            Solution::three_sum(nums),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 2, -2, -1];
        // -2 -1 1 2
        assert_eq!(Solution::three_sum(nums), vec![] as Vec<Vec<i32>>);
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
