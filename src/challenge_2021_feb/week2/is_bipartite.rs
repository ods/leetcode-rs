// Is Graph Bipartite?
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3639/

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut which = vec![0; graph.len()];
        let mut start = 0;
        loop {
            let mut deque = VecDeque::new();
            match graph[start..]
                .iter()
                .enumerate()
                .filter(|(idx, edges)| {
                    !edges.is_empty() && which[start + idx] == 0
                })
                .next()
            {
                None => {
                    return true;
                }
                Some((idx, _)) => {
                    start += idx;
                    which[start] = 1;
                    deque.push_back(start);
                }
            }
            while let Some(idx) = deque.pop_front() {
                let set = which[idx];
                for node in &graph[idx] {
                    let node = *node as usize;
                    if which[node] == set {
                        return false;
                    } else if which[node] == 0 {
                        which[node] = -set;
                        deque.push_back(node);
                    }
                }
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::is_bipartite(vec![
                vec![1, 3],
                vec![0, 2],
                vec![1, 3],
                vec![0, 2]
            ]),
            true,
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::is_bipartite(vec![
                vec![1, 2, 3],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 2]
            ]),
            false,
        )
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::is_bipartite(vec![vec![]]), true)
    }

    #[test]
    fn test_even_circle() {
        let mut graph = vec![vec![0_i32; 2]; 100];
        for i in 0..100 {
            graph[i][0] = (i + 99).rem_euclid(100) as _;
            graph[i][1] = (i + 1).rem_euclid(100) as _;
        }
        assert_eq!(Solution::is_bipartite(graph), true)
    }

    #[test]
    fn test_odd_circle() {
        let mut graph = vec![vec![0_i32; 2]; 99];
        for i in 0..99 {
            graph[i][0] = (i + 98).rem_euclid(99) as _;
            graph[i][1] = (i + 1).rem_euclid(99) as _;
        }
        assert_eq!(Solution::is_bipartite(graph), false)
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::is_bipartite(vec![
                vec![3],
                vec![3],
                vec![],
                vec![0, 1],
                vec![6],
                vec![],
                vec![4],
                vec![9],
                vec![],
                vec![7],
            ]),
            true
        )
    }
}
