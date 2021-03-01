// Minimum Operations to Reduce X to Zero
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3603/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let vec_len = nums.len();

        let new_min_len = |min_len, left, right| {
            let new_len = (left + vec_len - 1 - right) as i32;
            if min_len == -1 || new_len < min_len {
                new_len
            } else {
                min_len
            }
        };

        let mut left = 0; // exclusive bound
        let mut right = nums.len() - 1; // exclusive bound
        let mut min_len = -1_i32;
        let mut sub = x;

        while left <= right && sub > 0 {
            sub -= nums[left];
            left += 1;
        }
        if sub == 0 {
            min_len = new_min_len(min_len, left, right)
        };
        if min_len == 1 {
            return 1;
        };

        while left > 0 && right >= left {
            while left > 0 && sub <= 0 {
                left -= 1;
                sub += nums[left];
                if sub == 0 {
                    min_len = new_min_len(min_len, left, right)
                };
                if min_len == 1 {
                    return 1;
                };
            }

            while right > left && sub > 0 {
                sub -= nums[right];
                right -= 1;
            }
            if sub == 0 {
                min_len = new_min_len(min_len, left, right)
            };
            if min_len == 1 {
                return 1;
            };
        }
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_operations(vec![1000, 1, 1, 2, 3], 1004), 3);
    }
}
