/*
 * @lc app=leetcode.cn id=100 lang=rust
 *
 * [100] 相同的树
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
// pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (_, None) => false,
            (None, _) => false,
            (Some(p), Some(q)) => {
                if p.borrow().val == q.borrow().val {
                    let p_left = p.borrow_mut().left.take();
                    let q_left = q.borrow_mut().left.take();
                    let p_right = p.borrow_mut().right.take();
                    let q_right = q.borrow_mut().right.take();
                    Self::is_same_tree(p_left, q_left) && Self::is_same_tree(p_right, q_right)
                } else {
                    false
                }
            }
        }
    }
}
// @lc code=end
