// Merge k Sorted Lists
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3615/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn vec_to_list(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter()
        .rev()
        .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}

fn list_next(list: &mut Option<Box<ListNode>>) -> Option<i32> {
    match list.take() {
        None => None,
        Some(inner) => {
            *list = inner.next;
            Some(inner.val)
        }
    }
}

fn list_to_vec(list: &mut Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    while let Some(val) = list_next(list) {
        res.push(val);
    }
    res
}

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::with_capacity(lists.len());
        for list in lists.iter_mut() {
            if list.is_some() {
                heap.push(list.take());
            }
        }
        let mut leader: Box<ListNode> = Box::new(ListNode::new(0));
        let mut current = &mut leader;
        while let Some(list) = heap.pop() {
            current.next = list;
            current = current.next.as_mut().unwrap();
            let next = current.next.take();
            if next.is_some() {
                heap.push(next)
            };
        }
        leader.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(src: &[&[i32]], expected: &[i32]) {
        let lists = src.iter().copied().map(vec_to_list).collect();
        let res = list_to_vec(&mut Solution::merge_k_lists(lists));
        assert_eq!(&res, expected);
    }

    #[test]
    fn exmaple1() {
        check(
            &[&[1, 4, 5], &[1, 3, 4], &[2, 6]],
            &[1, 1, 2, 3, 4, 4, 5, 6],
        );
    }

    #[test]
    fn exmaple2() {
        check(&[], &[]);
    }

    #[test]
    fn exmaple3() {
        check(&[&[]], &[]);
    }
}
