use crate::solution::Solution;
use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// 226. Invert Binary Tree - on-site modification - https://leetcode.com/problems/invert-binary-tree/description/
    pub fn invert_tree_on_site(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.left = Self::invert_tree_on_site(right);
            node.right = Self::invert_tree_on_site(left);
        }

        root
    }

    /// 226. Invert Binary Tree - creating a new tree - https://leetcode.com/problems/invert-binary-tree/description/
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let node = node.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left: Self::invert_tree(node.right.clone()),
                    right: Self::invert_tree(node.left.clone()),
                })))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_case_1() {
        let original_tree = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(7, node(6, None, None), node(9, None, None)),
        );
        let expected_tree = node(
            4,
            node(7, node(9, None, None), node(6, None, None)),
            node(2, node(3, None, None), node(1, None, None)),
        );
        assert_eq!(Solution::invert_tree_on_site(original_tree), expected_tree);
    }
    #[test]
    fn test_case_2() {
        let original_tree = node(2, node(1, None, None), node(3, None, None));
        let expected_tree = node(2, node(3, None, None), node(1, None, None));
        assert_eq!(Solution::invert_tree_on_site(original_tree), expected_tree);
    }
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::invert_tree_on_site(None), None);
    }

    #[test]
    fn test_case_4() {
        let original_tree = node(
            4,
            node(2, node(1, None, None), node(3, None, None)),
            node(7, node(6, None, None), node(9, None, None)),
        );
        let expected_tree = node(
            4,
            node(7, node(9, None, None), node(6, None, None)),
            node(2, node(3, None, None), node(1, None, None)),
        );
        assert_eq!(Solution::invert_tree(original_tree), expected_tree);
    }
    #[test]
    fn test_case_5() {
        let original_tree = node(2, node(1, None, None), node(3, None, None));
        let expected_tree = node(2, node(3, None, None), node(1, None, None));
        assert_eq!(Solution::invert_tree(original_tree), expected_tree);
    }
    #[test]
    fn test_case_6() {
        assert_eq!(Solution::invert_tree(None), None);
    }
}
