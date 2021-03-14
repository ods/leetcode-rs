// Swapping Nodes in a Linked List
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3671/

use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_nodes(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {
        let mut vec = Vec::new();
        while let Some(mut inner) = head {
            head = inner.next.take();
            vec.push(inner);
        }

        let idx1 = k as usize - 1;
        let idx2 = vec.len() - k as usize;
        if idx1 != idx2 {
            vec.swap(idx1, idx2);
        }

        let mut head = None;
        for mut inner in vec.into_iter().rev() {
            inner.next = head;
            head = Some(inner);
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(src: &[i32], k: i32, expected: &[i32]) {
        let res = Solution::swap_nodes(ListNode::from_slice(src), k);
        assert_eq!(ListNode::to_vec(&res), expected);
    }

    #[test]
    fn example1() {
        check(&[1, 2, 3, 4, 5], 2, &[1, 4, 3, 2, 5]);
    }

    #[test]
    fn example2() {
        check(
            &[7, 9, 6, 6, 7, 8, 3, 0, 9, 5],
            5,
            &[7, 9, 6, 6, 8, 7, 3, 0, 9, 5],
        );
    }

    #[test]
    fn example3() {
        check(&[1], 1, &[1]);
    }

    #[test]
    fn example4() {
        check(&[1, 2], 1, &[2, 1]);
    }

    #[test]
    fn example5() {
        check(&[1, 2, 3], 2, &[1, 2, 3]);
    }

    #[test]
    fn test_backward() {
        check(&[1, 2], 2, &[2, 1]);
    }
}
