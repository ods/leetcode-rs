// 3Sum With Multiplicity
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3682/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut one = HashMap::<i32, u64>::new();
        let mut two = HashMap::<i32, u64>::new();
        let mut total = 0;
        for num in arr {
            total += two.get(&(target - num)).unwrap_or(&0);
            for (other, count) in &one {
                *two.entry(num + other).or_default() += count;
            }
            *one.entry(num).or_default() += 1;
        }
        (total % 1_000_000_007) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8),
            20
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
    }

    #[test]
    fn test_big() {
        assert_eq!(Solution::three_sum_multi(vec![0; 3000], 0), 495_500_972);
    }
}
