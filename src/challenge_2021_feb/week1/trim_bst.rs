// Trim a Binary Search Tree
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3626/

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let val = node.borrow().val;
            if val < low {
                Solution::trim_bst(node.borrow().right.clone(), low, high)
            } else if val > high {
                Solution::trim_bst(node.borrow().left.clone(), low, high)
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: Solution::trim_bst(
                        node.borrow().left.clone(),
                        low,
                        high,
                    ),
                    right: Solution::trim_bst(
                        node.borrow().right.clone(),
                        low,
                        high,
                    ),
                })))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        node(val, None, None)
    }

    #[test]
    fn example1() {
        let root = node(1, leaf(0), leaf(2));
        let expected = node(1, None, leaf(2));
        assert_eq!(Solution::trim_bst(root, 1, 2), expected);
    }

    #[test]
    fn example2() {
        let root = node(3, node(0, None, node(2, leaf(1), None)), leaf(4));
        let expected = node(3, node(2, leaf(1), None), None);
        assert_eq!(Solution::trim_bst(root, 1, 3), expected);
    }

    #[test]
    fn example3() {
        let root = leaf(1);
        let expected = leaf(1);
        assert_eq!(Solution::trim_bst(root, 1, 2), expected);
    }

    #[test]
    fn example4() {
        let root = node(1, None, leaf(2));
        let expected = node(1, None, leaf(2));
        assert_eq!(Solution::trim_bst(root, 1, 3), expected);
    }

    #[test]
    fn example5() {
        let root = node(1, None, leaf(2));
        let expected = leaf(2);
        assert_eq!(Solution::trim_bst(root, 2, 4), expected);
    }
}
