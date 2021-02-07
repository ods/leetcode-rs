use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub const NULL: i32 = std::i32::MIN;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(
        values: Vec<Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let mut nodes = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        nodes.push_back(root.clone());
        let mut idx = 1;
        while let Some(current) = nodes.pop_front() {
            if let Some(&Some(value)) = values.get(idx) {
                let node = Rc::new(RefCell::new(TreeNode::new(value)));
                (&*current).borrow_mut().left.replace(node.clone());
                nodes.push_back(node);
            }
            idx += 1;
            if let Some(&Some(value)) = values.get(idx) {
                let node = Rc::new(RefCell::new(TreeNode::new(value)));
                (&*current).borrow_mut().right.replace(node.clone());
                nodes.push_back(node);
            }
            idx += 1;
        }
        Some(root)
    }

    pub fn from_arr(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::from_vec(
            values
                .iter()
                .map(|&v| if v == NULL { None } else { Some(v) })
                .collect(),
        )
    }
}

mod test {
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
    fn test1() {
        assert_eq!(TreeNode::from_arr(&[1, 0, 2]), node(1, leaf(0), leaf(2)));
    }

    #[test]
    fn test2() {
        assert_eq!(
            TreeNode::from_arr(&[3, 0, 4, NULL, 2, NULL, NULL, 1]),
            node(3, node(0, None, node(2, leaf(1), None)), leaf(4))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(TreeNode::from_arr(&[1]), leaf(1));
    }

    #[test]
    fn test4() {
        assert_eq!(TreeNode::from_arr(&[1, NULL, 2]), node(1, None, leaf(2)));
    }
}
