// Average of Levels in Binary Tree
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3661/

use crate::tree::TreeNode;

pub struct Solution;

use std::{cell::RefCell, rc::Rc};

impl Solution {
    fn visit(
        acc: &mut Vec<(f64, usize)>,
        node: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
    ) {
        if let &Some(ref inner) = node {
            let cell = inner.borrow();
            if level == acc.len() {
                acc.push((cell.val as f64, 1));
            } else {
                let (sum, count) = acc[level];
                acc[level] = (sum + cell.val as f64, count + 1);
            }
            Solution::visit(acc, &cell.right, level + 1);
            Solution::visit(acc, &cell.left, level + 1);
        }
    }

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut acc = Vec::new();
        Self::visit(&mut acc, &root, 0);
        acc.iter().map(|(sum, count)| sum / *count as f64).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::average_of_levels(node(
                3,
                leaf(9),
                node(20, leaf(15), leaf(7))
            )),
            vec![3.0, 14.5, 11.0],
        );
    }
}
