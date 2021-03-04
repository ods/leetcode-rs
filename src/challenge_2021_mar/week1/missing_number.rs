// Missing Number
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3659/

pub struct Solution;

impl Solution {
    #[cfg(disable)]
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = nums.len() as i32;
        for (idx, num) in nums.iter().enumerate() {
            res ^= idx as i32 ^ num;
        }
        res
    }

    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(nums.len() as i32, |res, (idx, num)| res ^ num ^ idx as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8,
        );
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }

    #[test]
    fn missing_zero() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
    }
}
