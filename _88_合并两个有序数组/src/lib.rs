/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 初始化三个指针：p1 指向 nums1有效元素的最后，p2 指向 nums2最后，p 指向nums1的最后即合并后的数组的最后
        let (mut p1, mut p2, mut p) = (m - 1, n - 1, m + n - 1);
        while p2 >= 0 {
            if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1 [p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p-=1;
        }
    }
}
// @lc code=end

// [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// 输出：[1,2,2,3,5,6]

// [2,0], m = 1, [1], n = 1 
// [1,2]

mod tests {
    use crate::Solution;

    #[test]
    fn test_my() {
        let mut nums1 = vec![2, 0];
        println!("TEST ON : {:?} {} {:?} {}", nums1, 1, vec![1], 1);
        Solution::merge(&mut nums1, 1, &mut vec![1], 1);
        assert_eq!(
            [1, 2].map(|e| e.to_string()).join(" "),
            nums1
                .iter_mut()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    #[test]
    fn test_my1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        println!("TEST ON : {:?} {} {:?} {}", nums1, 3, vec![2, 5, 6], 3);
        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(
            [1, 2, 2, 3, 5, 6].map(|e| e.to_string()).join(" "),
            nums1
                .iter_mut()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

// [1,2] 和 nums1 vec!创建出来的区别
// 一个是array， 一个是Vec 的类型
