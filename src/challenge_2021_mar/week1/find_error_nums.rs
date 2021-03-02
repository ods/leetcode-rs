// Set Mismatch
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3658/

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![0_u128; (nums.len() + 127) >> 7];
        let mut dup = 0;
        let mut missing = 0;
        for (idx, &num) in nums.iter().enumerate() {
            let block_idx = (num - 1) >> 7;
            let mask = 1 << ((num - 1) & 0x7f);
            let block = seen.get_mut(block_idx as usize).unwrap();
            if *block & mask != 0 {
                dup = num
            }
            *block |= mask;
            missing ^= num ^ (idx as i32 + 1);
        }
        vec![dup, missing ^ dup]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn test_missing_first() {
        assert_eq!(Solution::find_error_nums(vec![2, 2]), vec![2, 1]);
    }
}
