// Create Sorted Array through Instructions
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3599/

pub struct Solution;

use std::cmp::min;

const MODULO: i32 = 1_000_000_007;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut res = Vec::with_capacity(instructions.len());
        let mut cost: i32 = 0;
        for inst in instructions {
            let value = inst * 2;
            let lower = res.binary_search(&(value - 1)).unwrap_err();
            let upper = res.binary_search(&(value + 1)).unwrap_err();
            cost += min(lower, res.len() - upper) as i32;
            res.insert(upper, value);

            if cost > MODULO {
                cost %= MODULO;
            }
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::create_sorted_array([1, 5, 6, 2].into()), 1)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::create_sorted_array([1, 2, 3, 6, 5, 4].into()), 3)
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::create_sorted_array([1, 3, 3, 3, 2, 4, 2, 1, 2].into()),
            4
        )
    }

    #[test]
    fn test_big() {
        let instructions: Vec<i32> =
            [4, 5, 6].iter().cycle().take(90_000).copied().collect();
        Solution::create_sorted_array(instructions);
    }
}
