// Number of 1 Bits
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3625/

pub struct Solution;

// Cheating
#[cfg(disable)]
impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as _
    }
}

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(mut n: u32) -> i32 {
        // Brian Kernighan’s Algorithm
        let mut count = 0;
        while n > 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::hammingWeight(0b1011), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::hammingWeight(0b10000000), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::hammingWeight(0b11111111111111111111111111111101),
            31,
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::hammingWeight(0), 0);
    }
}
