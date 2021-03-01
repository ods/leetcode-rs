// Is Graph Bipartite?
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3639/

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut which = vec![0; graph.len()];
        for (idx, edges) in graph.iter().enumerate() {
            // Find first non-empty unvisited
            if which[idx] != 0 || edges.is_empty() {
                continue;
            };

            // Mark all reachable from it
            let mut deque = VecDeque::new();
            which[idx] = 1;
            deque.push_back(idx);
            while let Some(idx) = deque.pop_front() {
                let set = which[idx];
                for next in &graph[idx] {
                    let next = *next as usize;
                    if which[next] == set {
                        return false;
                    } else if which[next] == 0 {
                        which[next] = -set;
                        deque.push_back(next);
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::is_bipartite(matrix![[1, 3], [0, 2], [1, 3], [0, 2]]),
            true,
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::is_bipartite(matrix![
                [1, 2, 3],
                [0, 2],
                [0, 1, 3],
                [0, 2]
            ]),
            false,
        )
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::is_bipartite(matrix![]), true)
    }

    #[test]
    fn test_even_circle() {
        let mut graph = matrix![0_i32; 2; 100];
        for i in 0..100 {
            graph[i][0] = (i + 99).rem_euclid(100) as _;
            graph[i][1] = (i + 1).rem_euclid(100) as _;
        }
        assert_eq!(Solution::is_bipartite(graph), true)
    }

    #[test]
    fn test_odd_circle() {
        let mut graph = matrix![0_i32; 2; 99];
        for i in 0..99 {
            graph[i][0] = (i + 98).rem_euclid(99) as _;
            graph[i][1] = (i + 1).rem_euclid(99) as _;
        }
        assert_eq!(Solution::is_bipartite(graph), false)
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::is_bipartite(matrix![
                [3],
                [3],
                [],
                [0, 1],
                [6],
                [],
                [4],
                [9],
                [],
                [7],
            ]),
            true,
        )
    }
}
