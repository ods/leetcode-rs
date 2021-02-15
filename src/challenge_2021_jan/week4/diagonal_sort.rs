// Sort the Matrix Diagonally
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3614/

struct Solution;

pub struct DiagIndexesIter {
    size: (usize, usize),
    i: usize,
    j: usize,
}

pub fn diag_indexes(size: (usize, usize), diag: usize) -> DiagIndexesIter {
    let (i, j) = if diag < size.0 {
        (size.0 - diag - 1, 0)
    } else {
        (0, diag - size.0 + 1)
    };
    DiagIndexesIter { size, i, j }
}

impl Iterator for DiagIndexesIter {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.size.0 || self.j >= self.size.1 {
            return None;
        }
        let res = (self.i, self.j);
        self.i += 1;
        self.j += 1;
        Some(res)
    }
}

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let size = (mat.len(), mat[0].len());
        for diag in 0..size.0 + size.1 - 1 {
            let mut diag_vals: Vec<_> =
                diag_indexes(size, diag).map(|(i, j)| mat[i][j]).collect();
            diag_vals.sort_unstable();
            diag_indexes(size, diag)
                .zip(diag_vals)
                .for_each(|((i, j), val)| mat[i][j] = val);
        }
        mat
    }
}

mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::diagonal_sort(matrix![
                [3, 3, 1, 1],
                [2, 2, 1, 2],
                [1, 1, 1, 2],
            ]),
            matrix![[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]],
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::diagonal_sort(matrix![
                [11, 25, 66, 1, 69, 7],
                [23, 55, 17, 45, 15, 52],
                [75, 31, 36, 44, 58, 8],
                [22, 27, 33, 25, 68, 4],
                [84, 28, 14, 11, 5, 50],
            ]),
            matrix![
                [5, 17, 4, 1, 52, 7],
                [11, 11, 25, 45, 8, 69],
                [14, 23, 25, 44, 58, 15],
                [22, 27, 31, 36, 50, 66],
                [84, 28, 75, 33, 55, 68],
            ],
        )
    }
}
