/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 */

use std::borrow::BorrowMut;

struct Solution;
// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Iterator for ListNode {
    type Item = Self;
    fn next(&mut self) -> Option<Box<ListNode>> {
        self.next 
    }
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //  1 1 2
        //  1 2 2 3 3 3
        //  1 1
        let head_iter = head.into_iter();
        // .fold(zero, |acc, &x| {
        //     format!("({acc} + {x})")
        // });
        // 需要存 same key 和 left box
        head_iter.fold((0,head_iter.next()), ||)
    //     head.into_iter()
    //         .fold((0, head.as_mut()), |(max, node)| match node.as_mut() {
    //             Some(node) => (std::cmp::max(
    // max, node.val 
    //             ), node),
    //             None => {}
    //         })
    }
}
// @lc code=end


// typescript
// function deleteDuplicates(head: ListNode | null): ListNode | null {
//     let p = head; 
//     while (p!= null) {
//         while (p.val == p.next?.val) {
//             p.next = p.next.next;
//         }
//         p = p.next 
//     }
//     return head
// };


