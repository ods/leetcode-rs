// Russian Doll Envelopes
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3690/

pub struct Solution;

use std::{
    cmp::Reverse,
    collections::BTreeSet,
    ops::Bound::{Excluded, Unbounded},
};

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // Reverse by height to disconnect envelopes with the same width
        envelopes.sort_unstable_by_key(|env| (env[0], Reverse(env[1])));
        // The set of values for the longest increasing subsequence
        let mut lis = BTreeSet::<i32>::new();
        for env in envelopes {
            let height = env[1];
            if lis.insert(height) {
                // Replace the first value bigger than one we just inserted
                lis.range((Excluded(height), Unbounded))
                    .next()
                    .copied()
                    .map(|height| lis.remove(&height));
            };
        }
        lis.len() as _
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
