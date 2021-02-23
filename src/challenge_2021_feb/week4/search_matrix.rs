// Search a 2D Matrix II
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3650/

struct Solution;

use std::{cmp::Ordering::*, collections::BinaryHeap};

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = 0;
        let mut j = 0;
        while i < m - 1 && matrix[i][n - 1] < target {
            i += 1;
        }
        while j < n - 1 && matrix[m - 1][j] < target {
            j += 1;
        }
        let mut heap = BinaryHeap::new();
        heap.push((-matrix[i][j], i, j));
        while let Some((val, i, j)) = heap.pop() {
            match target.cmp(&-val) {
                Less => return false,
                Equal => return true,
                _ => {}
            }
            if i < m - 1 {
                heap.push((-matrix[i + 1][j], i + 1, j));
            }
            if j < n - 1 {
                heap.push((-matrix[i][j + 1], i, j + 1));
            }
        }
        false
    }
}

mod test {
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
}
