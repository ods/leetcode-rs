// Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3669/

pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let s = s.as_bytes();

        let mut needed: usize = 1 << k;
        if s.len() < k + needed - 1 {
            return false;
        }

        // Bit vector, with bits for required values set
        let mut n_blocks = needed / 128;
        let rem = needed % 128;
        if rem != 0 {
            n_blocks += 1;
        }
        let mut unseen = vec![std::u128::MAX; n_blocks];
        if rem != 0 {
            unseen[n_blocks - 1] = (1_u128 << rem) - 1;
        }

        let mut val = s[..k - 1]
            .iter()
            .fold(0, |acc, &b| (acc << 1) + (b == b'1') as u128);

        let mask = (1_u128 << k) - 1;
        for &b in s[k - 1..].iter() {
            val = ((val << 1) + (b == b'1') as u128) & mask;
            let unseen_bit = unseen[val as usize / 128] & (1 << val % 128);
            if unseen_bit != 0 {
                unseen[val as usize / 128] -= unseen_bit;
                needed -= 1;
                if needed == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::has_all_codes("00110110".into(), 2), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::has_all_codes("00110".into(), 2), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::has_all_codes("0110".into(), 1), true);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::has_all_codes("0110".into(), 2), false);
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::has_all_codes("0000000001011100".into(), 4),
            false
        );
    }

    #[test]
    fn test_big() {
        assert_eq!(Solution::has_all_codes("10010".repeat(10_000), 20), false);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::has_all_codes("00000000001011100".into(), 3),
            true
        );
    }
}
