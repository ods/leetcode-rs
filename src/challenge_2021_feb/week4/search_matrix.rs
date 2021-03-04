// Search a 2D Matrix II
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3650/

pub struct Solution;

use std::cmp::Ordering::*;

#[cfg(disable)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = m - 1;
        let mut j = 0;
        while j < n {
            match matrix[i][j].cmp(&target) {
                Equal => return true,
                Greater if i > 0 => {
                    i -= 1;
                }
                _ => j += 1,
            }
        }
        false
    }
}

use std::ops::Range;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        Self::search_block(&matrix, target, &(0..m), &(0..n))
    }

    fn search_block(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        ir: &Range<usize>,
        jr: &Range<usize>,
    ) -> bool {
        // `is_empty` doesn't work in Leetcode's version, the replacement could
        // be: `if ir.start == ir.end || jr.start == jr.end`
        if ir.is_empty() || jr.is_empty() {
            return false;
        }
        let im = (ir.start + ir.end) / 2;
        let jm = (jr.start + jr.end) / 2;
        match matrix[im][jm].cmp(&target) {
            Equal => true,
            Less => {
                Self::search_block(
                    matrix,
                    target,
                    &(ir.start..im + 1),
                    &(jm + 1..jr.end),
                ) || Self::search_block(matrix, target, &(im + 1..ir.end), jr)
            }
            Greater => {
                Self::search_block(matrix, target, &(ir.start..im), jr)
                    || Self::search_block(
                        matrix,
                        target,
                        &(im..ir.end),
                        &(jr.start..jm),
                    )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::search_matrix(
                matrix![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30],
                ],
                5,
            ),
            true,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::search_matrix(
                matrix![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30],
                ],
                20,
            ),
            false,
        );
    }

    #[test]
    fn test_big() {
        let mut matrix = matrix![0_i32; 300; 300];
        for i in 0..300 {
            for j in 0..300 {
                matrix[i][j] = (i + j) as i32;
            }
        }
        assert_eq!(Solution::search_matrix(matrix, 500), true);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::search_matrix(matrix![[-5]], -5), true);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::search_matrix(matrix![[-5]], -10), false);
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::search_matrix(
                matrix![
                    [1, 2, 3, 4, 5],
                    [6, 7, 8, 9, 10],
                    [11, 12, 13, 14, 15],
                    [16, 17, 18, 19, 20],
                    [21, 22, 23, 24, 25],
                ],
                5
            ),
            true,
        );
    }
}
