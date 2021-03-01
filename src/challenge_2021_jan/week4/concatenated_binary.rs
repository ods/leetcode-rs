// Concatenation of Consecutive Binary Numbers
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3618/

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut nbits = 1;
        let mut res: u64 = 0;
        for i in 1..=n as u64 {
            res = ((res << nbits) + i) % 1_000_000_007;
            if (i & (i + 1)) == 0 {
                nbits += 1;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::concatenated_binary(1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::concatenated_binary(3), 27);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }
}
