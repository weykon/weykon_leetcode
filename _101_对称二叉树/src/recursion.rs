// 递归
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn mirror(
    lefe_tree: Option<Rc<RefCell<TreeNode>>>,
    right_tree: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (lefe_tree, right_tree) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            if l.borrow().val == r.borrow().val {
                let ll = l.borrow_mut().left.take();
                let lr = l.borrow_mut().right.take();
                let rl = r.borrow_mut().left.take();
                let rr = r.borrow_mut().right.take();
                mirror(ll, rr) && mirror(lr, rl)
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn recursion_fn(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(root) => {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            mirror(left, right)
        }
    }
}
