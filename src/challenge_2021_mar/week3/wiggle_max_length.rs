// Wiggle Subsequence
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3676/

pub struct Solution;

enum State {
    Any(i32),
    Min(i32),
    Max(i32),
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        use State::*;
        let mut state = Any(nums[0]);
        let mut count = 1;
        for &num in nums[1..].iter() {
            match state {
                Any(prev) | Min(prev) if num > prev => {
                    state = Max(num);
                    count += 1;
                }
                Min(prev) if num < prev => {
                    state = Min(num);
                }
                Any(prev) | Max(prev) if num < prev => {
                    state = Min(num);
                    count += 1;
                }
                Max(prev) if num > prev => {
                    state = Max(num);
                }
                _ => {}
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(Solution::wiggle_max_length(vec![1]), 1);
    }

    #[test]
    fn test_two_eq() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 1]), 1);
    }

    #[test]
    fn test_two_ne() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 2]), 2);
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::wiggle_max_length(vec![
                1, 17, 5, 10, 13, 15, 10, 5, 16, 8
            ]),
            7
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
}
