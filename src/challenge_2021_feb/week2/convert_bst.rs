// Convert BST to Greater Tree
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3634/

use crate::tree::{TreeNode, NULL};
use std::{cell::RefCell, rc::Rc};

struct Solution;

impl Solution {
    pub fn convert_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut val = 0;
        Self::visit(&mut root, &mut val);
        root
    }
    fn visit(root: &mut Option<Rc<RefCell<TreeNode>>>, val: &mut i32) {
        if let Some(inner) = root {
            let mut node = inner.borrow_mut();
            Self::visit(&mut node.right, val);
            *val += node.val;
            node.val = *val;
            Self::visit(&mut node.left, val);
        }
    }
}

mod test {
    use super::*;

    fn check(root: &[i32], expected: &[i32]) {
        let root = TreeNode::from_arr(root);
        let expected = TreeNode::from_arr(expected);
        let res = Solution::convert_bst(root);
        assert_eq!(res, expected)
    }

    #[test]
    fn example1() {
        check(
            &[
                4, 1, 6, 0, 2, 5, 7, NULL, NULL, NULL, 3, NULL, NULL, NULL, 8,
            ],
            &[
                30, 36, 21, 36, 35, 26, 15, NULL, NULL, NULL, 33, NULL, NULL,
                NULL, 8,
            ],
        )
    }

    #[test]
    fn example2() {
        check(&[0, NULL, 1], &[1, NULL, 1]);
    }

    #[test]
    fn example3() {
        check(&[1, 0, 2], &[3, 3, 2]);
    }

    #[test]
    fn example4() {
        check(&[3, 2, 4, 1], &[7, 9, 4, 10]);
    }
}
