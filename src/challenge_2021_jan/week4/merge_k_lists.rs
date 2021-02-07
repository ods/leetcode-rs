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

fn rev_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut current_inner) = list.take() {
        list = current_inner.next.take();
        current_inner.next = prev;
        prev = Some(current_inner);
    }
    prev.take()
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

use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for (idx, list) in lists.iter_mut().enumerate() {
            if let Some(val) = list_next(list) {
                heap.push((-val, idx));
            }
        }
        let mut res = None;
        while let Some((nval, idx)) = heap.pop() {
            res = Some(Box::new(ListNode {
                val: -nval,
                next: res,
            }));
            if let Some(val) = list_next(&mut lists[idx]) {
                heap.push((-val, idx));
            }
        }
        rev_list(res)
    }
}

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

    #[test]
    fn test_rev() {
        assert_eq!(rev_list(vec_to_list(&[1, 2, 3])), vec_to_list(&[3, 2, 1]));
    }
}
