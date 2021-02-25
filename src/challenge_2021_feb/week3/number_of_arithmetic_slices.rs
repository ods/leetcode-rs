// Arithmetic Slices
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3644/

struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }
        let mut count = 0;
        let mut diffs = a.windows(2).map(|pair| pair[1] - pair[0]);
        let mut cur_diff = diffs.next().unwrap();
        let mut cur_len = 1;
        for diff in diffs {
            if diff == cur_diff {
                cur_len += 1;
            } else {
                if cur_len >= 2 {
                    count += (cur_len - 1) * cur_len / 2;
                }
                cur_diff = diff;
                cur_len = 1;
            }
        }
        if cur_len >= 2 {
            count += (cur_len - 1) * cur_len / 2;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn test_small() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2]), 0);
    }

    #[test]
    fn test_adjacent() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 2, 1]),
            2,
        );
    }

    #[test]
    fn test_all_different() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 4, 8]), 0);
    }
}
