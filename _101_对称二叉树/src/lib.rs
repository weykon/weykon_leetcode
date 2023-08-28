/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
 */
mod iter;
mod recursion;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn cmp(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                if left.borrow().val == right.borrow().val {
                    let ll = left.borrow_mut().left.take();
                    let lr = left.borrow_mut().right.take();
                    let rr = right.borrow_mut().right.take();
                    let rl = right.borrow_mut().left.take();
                    Self::cmp(ll, rr) && Self::cmp(lr, rl)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                Self::cmp(left, right)
            }
        }
    }
}
// @lc code=end

// 递归和迭代

mod tests {
    use crate::recursion;

    #[test]
    pub fn test_my() {
        let root = [1, 2, 2, 3, 4, 4, 3];

        // assert_eq!(recursion::recursion_fn(root), true);
    }
}

// fn create_tree_from_arr(arr: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//     fn create_one_tree(
//         val: Option<i32>,
//         l: Option<i32>,
//         r: Option<i32>,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         if let Some(val) = val {
//             let tree = Rc::new(RefCell::new(TreeNode::new(val)));
//             if let Some(left) = l {
//                 tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(left))));
//             }
//             if let Some(right) = r {
//                 tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(right))));
//             }
//             Some(tree)
//         } else {
//             return None;
//         }
//     }

// }
