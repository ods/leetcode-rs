// Number of Steps to Reduce a Number to Zero
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3637/

struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps = 0;
        while num != 0 {
            steps += 1;
            if (num & 1) == 1 {
                num -= 1;
            } else {
                num >>= 1;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::number_of_steps(0), 0);
    }
}
