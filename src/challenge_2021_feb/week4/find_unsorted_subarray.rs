// Shortest Unsorted Continuous Subarray
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3652/

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut start =
            match (0..len - 1).find(|&idx| nums[idx + 1] < nums[idx]) {
                Some(idx) => idx,
                None => return 0,
            };
        let mut end = match (1..len).rfind(|&idx| nums[idx - 1] > nums[idx]) {
            Some(idx) => idx,
            None => return 0,
        };
        let min = *nums[start..=end].iter().min().unwrap();
        let max = *nums[start..=end].iter().max().unwrap();
        while start > 0 && nums[start - 1] > min {
            start -= 1;
        }
        while end < len - 1 && nums[end + 1] < max {
            end += 1;
        }
        (end - start + 1) as _
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::find_unsorted_subarray(vec![2, 1]), 2);
    }
}
