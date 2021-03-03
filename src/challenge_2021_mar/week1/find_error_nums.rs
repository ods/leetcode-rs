// Set Mismatch
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3658/

struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut dup = 0;
        let mut missing = 0;
        for idx in 0..nums.len() {
            missing ^= idx as i32 + 1;
            let num = nums[idx].abs();
            let seen = nums.get_mut(num as usize - 1).unwrap();
            if *seen > 0 {
                *seen = -*seen;
                missing ^= num;
            } else {
                dup = num;
            }
        }
        vec![dup, missing]
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
