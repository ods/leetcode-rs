use crate::tree::{TreeNode, NULL};
use std::{cell::RefCell, rc::Rc};

struct Solution;

impl Solution {
    fn visit(
        res: &mut Vec<i32>,
        node: &Option<Rc<RefCell<TreeNode>>>,
        idx: usize,
    ) {
        if let &Some(ref inner) = node {
            let cell = inner.borrow();
            if idx >= res.len() {
                res.push(cell.val);
            }
            Solution::visit(res, &cell.right, idx + 1);
            Solution::visit(res, &cell.left, idx + 1);
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Solution::visit(&mut res, &root, 0);
        res
    }
}

mod test {
    use super::*;

    fn check(src: &[i32], expected: &[i32]) {
        assert_eq!(
            Solution::right_side_view(TreeNode::from_arr(src)),
            expected.to_vec()
        );
    }

    #[test]
    fn example() {
        check(&[1, 2, 3, NULL, 5, NULL, 4], &[1, 3, 4]);
    }
}
