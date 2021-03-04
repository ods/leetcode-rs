// Vertical Order Traversal of a Binary Tree
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3621/

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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

struct Vertical {
    neg: Vec<Vec<(i32, i32)>>,
    not_neg: Vec<Vec<(i32, i32)>>,
}

impl Vertical {
    pub fn from_tree(node: &TreeNode) -> Self {
        let mut res = Self {
            neg: vec![],
            not_neg: vec![],
        };
        res.traverse(node, 0, 0);
        res
    }
    fn traverse(&mut self, node: &TreeNode, x: i32, y: i32) {
        if x < 0 {
            if -x as usize > self.neg.len() {
                self.neg.push(vec![]);
            };
            self.neg[(-x - 1) as usize].push((-y, node.val));
        } else {
            if x as usize >= self.not_neg.len() {
                self.not_neg.push(vec![]);
            };
            self.not_neg[x as usize].push((-y, node.val));
        }
        if let Some(left) = node.left.as_ref() {
            self.traverse(&*left.borrow(), x - 1, y - 1);
        }
        if let Some(right) = node.right.as_ref() {
            self.traverse(&*right.borrow(), x + 1, y - 1);
        }
    }
    fn to_vec(self) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(self.neg.len() + self.not_neg.len());
        for mut v in self.neg.into_iter().rev().chain(self.not_neg.into_iter())
        {
            v.sort_unstable();
            res.push(v.into_iter().map(|(_, val)| val).collect());
        }
        res
    }
}

impl Solution {
    pub fn vertical_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Vec<i32>> {
        Vertical::from_tree(&*root.unwrap().borrow()).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    fn cell(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn example1() {
        let root = cell(
            3,
            cell(9, None, None),
            cell(20, cell(15, None, None), cell(7, None, None)),
        );
        assert_eq!(
            Solution::vertical_traversal(root),
            matrix![[9], [3, 15], [20], [7]]
        );
    }

    #[test]
    fn example2() {
        let root = cell(
            1,
            cell(2, cell(4, None, None), cell(5, None, None)),
            cell(3, cell(6, None, None), cell(7, None, None)),
        );
        assert_eq!(
            Solution::vertical_traversal(root),
            matrix![[4], [2], [1, 5, 6], [3], [7]]
        );
    }
}
