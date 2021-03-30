// Russian Doll Envelopes
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3690/

pub struct Solution;

impl Solution {
    // First accepted, but slow
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable();
        let mut seen = std::collections::BTreeMap::<(i32, i32), i32>::new();
        let mut max_len = 0;
        for env in envelopes {
            let len = seen.range(..(env[1], 0)).rev().fold(
                0,
                |max_len, (&(_, x), &len)| {
                    if x < env[0] {
                        max_len.max(len)
                    } else {
                        max_len
                    }
                },
            ) + 1;
            max_len = max_len.max(len);
            seen.insert((env[1], env[0]), len);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::max_envelopes(matrix![[5, 4], [6, 4], [6, 7], [2, 3]]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::max_envelopes(matrix![[1, 1], [1, 1], [1, 1]]),
            1
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::max_envelopes(matrix![
                [4, 5],
                [4, 6],
                [6, 7],
                [2, 3],
                [1, 1]
            ]),
            4
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::max_envelopes(matrix![
                [15, 8],
                [2, 20],
                [2, 14],
                [4, 17],
                [8, 19],
                [8, 9],
                [5, 7],
                [11, 19],
                [8, 11],
                [13, 11],
                [2, 13],
                [11, 19],
                [8, 11],
                [13, 11],
                [2, 13],
                [11, 19],
                [16, 1],
                [18, 13],
                [14, 17],
                [18, 19]
            ]),
            5
        );
    }

    #[test]
    fn test_big() {
        let envelopes = (1..=5000).map(|n| vec![n, n]).collect();
        assert_eq!(Solution::max_envelopes(envelopes), 5000);
    }
}
