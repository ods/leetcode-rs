// Max Number of K-Sum Pairs
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3608/

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut stack = std::collections::HashMap::new();
        let mut count = 0;
        for num in nums {
            if num >= k {
                continue;
            }
            let complement = k - num;
            if let Some(c) = stack.get_mut(&complement) {
                if *c == 1 {
                    stack.remove(&complement);
                } else {
                    *c -= 1;
                }
                count += 1;
            } else {
                stack.entry(num).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        count
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_operations(vec![2, 2, 2, 3, 1, 1, 4, 1], 4),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_operations(
                vec![
                    2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2
                ],
                3
            ),
            4
        );
    }
}
