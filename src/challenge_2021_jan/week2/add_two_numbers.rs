// Add Two Numbers
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3601/

use std::iter::successors;

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

pub fn list_from_num(num: i64) -> Option<Box<ListNode>> {
    successors(Some((num % 10, num / 10)), |&(_, num)| {
        if num == 0 {
            None
        } else {
            Some((num % 10, num / 10))
        }
    })
    .map(|(rem, _)| rem as i32)
    .collect::<Vec<i32>>()
    .iter()
    .rev()
    .fold(None, |head, &digit| {
        Some(Box::new(ListNode {
            val: digit,
            next: head,
        }))
    })
}

pub fn num_from_list(l: Option<Box<ListNode>>) -> i64 {
    successors(l.as_ref(), |&node| node.next.as_ref())
        .fold((0_i64, 1), |(num, mult), node| {
            (num + node.val as i64 * mult, mult * 10)
        })
        .0
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head1 = l1.as_ref();
        let mut head2 = l2.as_ref();
        let mut digits: Vec<i32> = Vec::new();
        let mut rem = 0;
        while head1 != None || head2 != None {
            if let Some(ref node) = head1 {
                rem += node.val;
                head1 = node.next.as_ref();
            }
            if let Some(ref node) = head2 {
                rem += node.val;
                head2 = node.next.as_ref();
            }
            digits.push(rem % 10);
            rem /= 10;
        }
        if rem != 0 {
            digits.push(rem);
        }
        digits.iter().rev().fold(None, |head, &digit| {
            Some(Box::new(ListNode {
                val: digit,
                next: head,
            }))
        })
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::add_two_numbers(list_from_num(342), list_from_num(465)),
            list_from_num(807),
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0)))
            ),
            Some(Box::new(ListNode::new(0))),
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::add_two_numbers(
                list_from_num(9999999),
                list_from_num(9999),
            ),
            list_from_num(10009998),
        )
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::new(3))),
                Some(Box::new(ListNode::new(0)))
            ),
            Some(Box::new(ListNode::new(3))),
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            num_from_list(Solution::add_two_numbers(
                list_from_num(942),
                list_from_num(9465)
            )),
            10407,
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            num_from_list(Solution::add_two_numbers(
                list_from_num(9),
                list_from_num(9999999991)
            )),
            10000000000,
        )
    }

    #[test]
    fn list_from_num0() {
        assert_eq!(
            list_from_num(0),
            Some(Box::new(ListNode { val: 0, next: None })),
        )
    }

    #[test]
    fn list_from_num1() {
        assert_eq!(
            list_from_num(12),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            })),
        )
    }

    #[test]
    fn test_round() {
        assert_eq!(num_from_list(list_from_num(942)), 942);
    }
}
