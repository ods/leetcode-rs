// Add One Row to Tree
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3666/

use crate::tree::TreeNode;

pub struct Solution;

use std::{cell::RefCell, rc::Rc};

impl Solution {
    fn visit(node: &mut Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) {
        if let Some(inner) = node {
            let mut cell = inner.borrow_mut();
            if d == 1 {
                cell.left = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: cell.left.take(),
                    right: None,
                })));
                cell.right = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: None,
                    right: cell.right.take(),
                })));
            } else {
                Self::visit(&mut cell.left, v, d - 1);
                Self::visit(&mut cell.right, v, d - 1);
            }
        }
    }

    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: root,
            right: None,
        })));
        Self::visit(&mut node, v, d);
        node.unwrap().borrow_mut().left.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::*;

    fn check(values: &[i32], v: i32, d: i32, expected: &[i32]) {
        assert_eq!(
            Solution::add_one_row(TreeNode::from_arr(values), v, d,),
            TreeNode::from_arr(expected),
        )
    }

    #[test]
    fn example1() {
        check(
            &[4, 2, 6, 3, 1, 5],
            1,
            2,
            &[4, 1, 1, 2, NULL, NULL, 6, 3, 1, 5],
        )
    }

    #[test]
    fn example2() {
        check(
            &[4, 2, NULL, 3, 1],
            1,
            3,
            &[4, 2, NULL, 1, 1, 3, NULL, NULL, 1],
        )
    }

    #[test]
    fn test_empty() {
        check(&[], 123, 1, &[123]);
    }

    #[test]
    fn test_d1() {
        check(&[1, 2, 3], 123, 1, &[123, 1, NULL, 2, 3]);
    }
}
