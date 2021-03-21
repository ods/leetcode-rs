// Reordered Power of 2
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3679/

pub struct Solution;

impl Solution {
    fn sorted_digits(mut n: u32) -> Vec<u8> {
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.sort_unstable();
        digits
    }

    pub fn reordered_power_of2(n: i32) -> bool {
        let n_digits = Self::sorted_digits(n as _);
        let mut cand =
            10_u32.pow(n_digits.len() as u32 - 1).next_power_of_two();
        loop {
            let cand_digits = Self::sorted_digits(cand);
            if cand_digits.len() > n_digits.len() {
                break false;
            } else if cand_digits == n_digits {
                break true;
            }
            cand <<= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reordered_power_of2(1), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reordered_power_of2(10), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::reordered_power_of2(16), true);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::reordered_power_of2(24), false);
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::reordered_power_of2(46), true);
    }
}
