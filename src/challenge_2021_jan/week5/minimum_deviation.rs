// Minimize Deviation in Array
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3622/

struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut max = 0;
        for num in nums {
            let mut base = num;
            while base & 1 == 0 {
                base >>= 1;
            }
            if base > max {
                max = base;
            }
            heap.push((-base, -num));
        }
        let min = -heap.peek().unwrap().0;
        let mut dev = max - min;
        loop {
            let (n_base, n_num) = heap.pop().unwrap();
            if n_base & 1 == 0 && n_base <= n_num {
                // We can't multiply anymore
                break dev;
            }
            let new_base = (-n_base) << 1;
            max = max.max(new_base);
            let min = new_base.min(-heap.peek().unwrap().0);
            dev = dev.min(max - min);
            heap.push((-new_base, n_num));
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::minimum_deviation(vec![10, 5, 10, 1]), 3);
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::minimum_deviation(vec![610, 778, 846, 733, 395]),
            236
        );
    }
}
