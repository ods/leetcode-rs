// Longest Harmonious Subsequence
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3628/

struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut max_len = 0;
        let mut prev_num = nums[0];
        let mut cur_num = prev_num;
        let mut prev_len = 0;
        let mut cur_len = 1;
        for &num in &nums[1..] {
            if num == cur_num {
                cur_len += 1;
            } else {
                if (cur_num - prev_num).abs() == 1
                    && prev_len + cur_len > max_len
                {
                    max_len = prev_len + cur_len;
                }
                prev_num = cur_num;
                cur_num = num;
                prev_len = cur_len;
                cur_len = 1;
            }
        }
        if (cur_num - prev_num).abs() == 1 && prev_len + cur_len > max_len {
            max_len = prev_len + cur_len;
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
