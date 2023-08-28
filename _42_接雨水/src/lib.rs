use std::ops::Deref;

/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */
// 假设了自己有信心但是没有把信心融入都操作中，也是不够
struct Solution;
// @lc code=start
impl Solution {
    pub fn trap(mut h: Vec<i32>) -> i32 {
        let mut water = 0;
        let l_max_vec = h.iter().fold((0, Vec::new()), |(max, mut vec), x| {
            let max = std::cmp::max(max, *x);
            vec.push(max.clone());
            (max, vec)
        });
        let r_max_vec = h
            .clone()
            .iter()
            .rev()
            .fold((0, Vec::new()), |(max, mut vec), x| {
                let max = std::cmp::max(max, *x);
                vec.push(max.clone());
                (max, vec)
            });

        // println!("l;{:?},\n r;{:?}", l_max_vec, r_max_vec);
        let l_iter = l_max_vec.1.iter();
        let r_iter = r_max_vec.1.iter();
        for (i, (l, r)) in l_iter.zip(r_iter.rev()).enumerate() {
            // println!("l;{}, r;{}", l, r);
            water += std::cmp::min(l, r) - h[i];
        }
        water
    }
}
// @lc code=end

fn iterator_fn(mut h: Vec<i32>) -> i32 {
    // 先来一个循环双指针
    // 这个我是有问题的，我虽然想到大概和排序比较相似，
    // 但是我排序相关的算法太差了，没想象出来。
    // 现在明白了
    let mut water = 0;
    let mut l = 0;
    let mut r = h.len() - 1;

    // 双端靠近
    while l < r {
        // 从变动的l和r下，像数组排序一样找高点
        if h[l] < h[r] {
            // 上面这个判断不要从一个定点去保留想法，而是变动的
            // r 的比较高，它保持不动，让l来动，找更高点

            // 然后顺便处理 l到r之间的池
            if h[l] > h[l + 1] {
                water += h[l] - h[l + 1];
                h[l + 1] = h[l]; // 这里直接懵了
                                 // 上面这条。是说，当进入这里，是在于l的右边还有比l高的也就是r。
                                 // 那么在这样的高度量，可以向右靠近让始终设想为作为最高边的同等最小最大高度(小于或等于)的边界
                                 // 那么，如果没有这么做，随着l的变动，它的这个高度就会丢失，而且当遇到了新高，l+1会自带同时刷新。
            }
            l += 1;
        } else {
            // 这里就是l高，l不动，r向左走
            if h[r] > h[r - 1] {
                water += h[r] - h[r - 1];
                h[r - 1] = h[r];
            }
            r -= 1;
        }
    }

    water
}

// 动态规划
fn dp(mut h: Vec<i32>) -> i32 {
    let mut water = 0;
    let h_iter = h.iter();
    let h_iter_clone = h.iter().clone();
    let l_max_vec = h_iter.fold((0, Vec::new()), |(max, mut vec), x| {
        let max = std::cmp::max(max, *x);
        vec.push(max.clone());
        (max, vec)
    });
    let r_max_vec = h_iter_clone
        .rev()
        .fold((0, Vec::new()), |(max, mut vec), x| {
            let max = std::cmp::max(max, *x);
            vec.push(max.clone());
            (max, vec)
        });

    // println!("l;{:?},\n r;{:?}", l_max_vec, r_max_vec);
    let l_iter = l_max_vec.1.iter();
    let r_iter = r_max_vec.1.iter();
    for (i, (l, r)) in l_iter.zip(r_iter.rev()).enumerate() {
        // println!("l;{}, r;{}", l, r);
        water += std::cmp::min(l, r) - h[i];
    }
    water
}

mod tests {
    use crate::{dp, Solution};

    #[test]
    fn tt() {
        let h = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

        assert_eq!(dp(h), 6);
    }
}
