/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */
pub struct Solution;
// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for num in nums.iter() {
            let value = map.entry(*num).or_insert(0);
            *value += 1;
        }
        let mut k: i32 = 0;
        let mut v: i32 = -1;
        for (&key, &value) in map.iter() {
            if value > v {
                k = key;
                v = value;
            }
        }
        return k;
    }
}
// @lc code=end

// 做法:
// hash，一次写hash循环，一次取值比较
// 排序后取中位数
// 摩尔投票法

// ::entry 用法
// #![allow(unused_variables)]
// fn main() {
// use std::collections::HashMap;

// let text = "hello world wonderful world";

// let mut map = HashMap::new();

// for word in text.split_whitespace() {
//     let count = map.entry(word).or_insert(0);
//     *count += 1;
// }

// println!("{:?}", map);
// }
