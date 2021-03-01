// Distribute Candies
// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3657/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let limit = candy_type.len() / 2;
        let mut unique = HashSet::with_capacity(limit);
        for item_type in candy_type {
            if unique.len() == limit {
                return limit as _;
            }
            unique.insert(item_type);
        }
        unique.len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
