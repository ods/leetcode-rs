// Shortest Path in Binary Matrix
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3638/

struct Solution;

use std::collections::BinaryHeap;

#[derive(Eq)]
struct Candidate {
    pub len: i32,
    pub r: usize,
    pub c: usize,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.len.cmp(&self.len))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let h = grid.len();
        let w = grid[0].len();
        let mut cands = BinaryHeap::new();
        if grid[0][0] == 0 {
            grid[0][0] = 1;
            cands.push(Candidate { len: 1, r: 0, c: 0 });
        }
        while let Some(cand) = cands.pop() {
            if cand.r == h - 1 && cand.c == w - 1 {
                return cand.len;
            }
            for r in (cand.r.max(1) - 1)..=(cand.r + 1).min(h - 1) {
                for c in (cand.c.max(1) - 1)..=(cand.c + 1).min(w - 1) {
                    if grid[r][c] == 0 {
                        grid[r][c] = 1;
                        cands.push(Candidate {
                            len: cand.len + 1,
                            r,
                            c,
                        });
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(matrix![[0, 1], [1, 0]]),
            2,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(matrix![
                [0, 0, 0],
                [1, 1, 0],
                [1, 1, 0],
            ]),
            4,
        );
    }

    #[test]
    fn test_blocked() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(matrix![
                [0, 1, 1],
                [1, 1, 1],
                [1, 1, 0],
            ]),
            -1,
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(matrix![
                [1, 0, 0],
                [1, 1, 0],
                [1, 1, 0],
            ]),
            -1,
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::shortest_path_binary_matrix(matrix![[0]]), 1);
    }

    #[test]
    fn test_big() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(matrix![0; 100; 100]),
            100,
        );
    }
}
