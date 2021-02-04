// Linked List Cycle
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3627/
// (Solutions in Rust are no accepted)

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

pub struct ListNode {
    pub val: i32,
    pub next: RefCell<Option<Rc<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: RefCell::new(None),
        }
    }
}

// impl Drop for ListNode {
//     fn drop(&mut self) {
//         println!("Dropping ListNode with val={}", self.val);
//     }
// }

impl Solution {
    fn has_cycle(list: Option<Rc<ListNode>>) -> bool {
        // Floyd’s Cycle-Finding Algorithm
        let mut follower = list.clone();
        let mut leader = match list {
            None => return false,
            Some(inner) => inner.next.borrow().clone(),
        };
        while let (Some(leader_inner), Some(follower_inner)) =
            (leader, follower)
        {
            if Rc::ptr_eq(&follower_inner, &leader_inner) {
                return true;
            };
            follower = follower_inner.next.borrow().clone();
            leader = match leader_inner.next.borrow().clone() {
                None => return false,
                Some(inner) => inner.next.borrow().clone(),
            };
        }
        false
    }
}

mod test {
    use super::*;

    fn from_arr(
        vals: &[i32],
        cycle_pos: i32,
    ) -> (Option<Rc<ListNode>>, Option<Rc<ListNode>>) {
        let mut first = None;
        let mut current = None;
        let mut cycle_node = None;
        for (pos, &val) in vals.iter().enumerate() {
            let node = Some(Rc::new(ListNode::new(val)));
            if cycle_pos >= 0 && pos == cycle_pos as usize {
                cycle_node = node.clone()
            }
            match current {
                None => {
                    first = node;
                    current = first.clone();
                }
                Some(inner) => {
                    inner.next.replace(node.clone());
                    current = node;
                }
            }
        }
        if cycle_node.is_some() {
            current.unwrap().next.replace(cycle_node.clone());
        }
        (first, cycle_node)
    }

    fn check(vals: &[i32], cycle_pos: i32) {
        let (list, cycle_node) = from_arr(vals, cycle_pos);
        let res = Solution::has_cycle(list);

        // Manually break cycle to avoid leaks
        if let Some(inner) = cycle_node {
            inner.next.replace(None);
        }

        assert_eq!(res, cycle_pos >= 0);
    }

    #[test]
    fn example1() {
        check(&[3, 2, 0, -4], 1);
    }

    #[test]
    fn example2() {
        check(&[1, 2], 0);
    }

    #[test]
    fn example3() {
        check(&[1], -1);
    }
}
