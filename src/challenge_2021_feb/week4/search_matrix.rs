// Search a 2D Matrix II
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3650/

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = m - 1;
        let mut j = 0;
        loop {
            let mut moved = false;
            while matrix[i][j] < target && j < n - 1 {
                j += 1;
                moved = true;
            }
            while matrix[i][j] > target && i > 0 {
                i -= 1;
                moved = true;
            }
            if matrix[i][j] == target {
                break true;
            } else if !moved {
                break false;
            }
        }
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
