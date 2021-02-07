// 48. Rotate Image
// https://leetcode.com/problems/rotate-image/

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix.len();
        let last = size - 1;
        let small_half = size / 2;
        let big_half = size - small_half;
        for r in 0..small_half {
            for c in 0..big_half {
                let tmp = matrix[r][c];
                matrix[r][c] = matrix[last - c][r];
                matrix[last - c][r] = matrix[last - r][last - c];
                matrix[last - r][last - c] = matrix[c][last - r];
                matrix[c][last - r] = tmp;
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn example2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ]
        );
    }

    #[test]
    fn example3() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }

    #[test]
    fn example4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
    }
}
