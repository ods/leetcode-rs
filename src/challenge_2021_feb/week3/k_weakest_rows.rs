// The K Weakest Rows in a Matrix
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3641/

struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(mat.len());
        for (idx, vec) in mat.iter().enumerate() {
            let weakness =
                vec.iter().position(|&item| item == 0).unwrap_or(vec.len());
            heap.push((-(weakness as i32), -(idx as i32)));
        }
        (0..k as usize).map(|_| -heap.pop().unwrap().1).collect()
    }
}

mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::k_weakest_rows(
                matrix![
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 0],
                    [1, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                ],
                3,
            ),
            vec![2, 0, 3],
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::k_weakest_rows(
                matrix![
                    [1, 0, 0, 0],
                    [1, 1, 1, 1],
                    [1, 0, 0, 0],
                    [1, 0, 0, 0],
                ],
                2,
            ),
            vec![0, 2],
        )
    }
}
