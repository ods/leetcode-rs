// Pacific Atlantic Water Flow
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3684/

pub struct Solution;

use std::{collections::HashSet, iter::FromIterator};

impl Solution {
    fn reachable(
        matrix: &Vec<Vec<i32>>,
        mut pool: Vec<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let h = matrix.len();
        let w = matrix[0].len();
        let mut res = HashSet::from_iter(pool.iter().copied());
        while let Some((r, c)) = pool.pop() {
            let height = matrix[r][c];
            if r > 0 && matrix[r - 1][c] >= height && res.insert((r - 1, c)) {
                pool.push((r - 1, c))
            };
            if r < h - 1
                && matrix[r + 1][c] >= height
                && res.insert((r + 1, c))
            {
                pool.push((r + 1, c))
            };
            if c > 0 && matrix[r][c - 1] >= height && res.insert((r, c - 1)) {
                pool.push((r, c - 1))
            };
            if c < w - 1
                && matrix[r][c + 1] >= height
                && res.insert((r, c + 1))
            {
                pool.push((r, c + 1))
            };
        }
        res
    }

    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let h = matrix.len();
        if h == 0 {
            return Vec::new();
        }
        let w = matrix[0].len();
        if w == 0 {
            return Vec::new();
        }
        let pacific = Self::reachable(
            &matrix,
            (0..w)
                .map(|c| (0, c))
                .chain((1..h).map(|r| (r, 0)))
                .collect(),
        );
        let atlantic = Self::reachable(
            &matrix,
            (0..w)
                .map(|c| (h - 1, c))
                .chain((0..h - 1).map(|r| (r, w - 1)))
                .collect(),
        );
        pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn example() {
        let mut res = Solution::pacific_atlantic(matrix![
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ]);
        res.sort_unstable();
        assert_eq!(
            res,
            matrix![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        )
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Solution::pacific_atlantic(matrix![]),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_empty_rows() {
        assert_eq!(
            Solution::pacific_atlantic(matrix![[]]),
            Vec::<Vec<i32>>::new()
        );
    }
}
