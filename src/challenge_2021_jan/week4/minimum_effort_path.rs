// Path With Minimum Effort
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3617/

use std::cmp::Ordering;

struct Solution;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Candidate {
    pos: (usize, usize),
    effort: i32,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.effort.cmp(&self.effort)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// My first solution
#[cfg(disable)]
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let finish = (heights.len() - 1, heights[0].len() - 1);
        let mut pos = (0, 0);
        let mut effort = 0;
        let mut reachable = HashMap::new();
        while pos != finish {
            // Mark as visited
            reachable.insert(pos, -1);

            // Update efforts for neighbors
            let height = heights[pos.0][pos.1];
            let update = {
                |reachable: &mut HashMap<(usize, usize), i32>, x, y| {
                    let val = reachable.entry((x, y)).or_insert(std::i32::MAX);
                    *val = (height - heights[x][y]).abs().min(*val);
                }
            };
            if pos.0 > 0 {
                update(&mut reachable, pos.0 - 1, pos.1);
            }
            if pos.1 > 0 {
                update(&mut reachable, pos.0, pos.1 - 1);
            }
            if pos.0 < finish.0 {
                update(&mut reachable, pos.0 + 1, pos.1);
            }
            if pos.1 < finish.1 {
                update(&mut reachable, pos.0, pos.1 + 1);
            }

            // Find the best next step
            let mut best = None;
            for (&next, &step_effort) in &reachable {
                if step_effort != -1 {
                    if step_effort <= effort {
                        // It's good enough
                        best = Some((next, step_effort));
                        break;
                    } else {
                        match best {
                            Some((_, best_effort))
                                if step_effort >= best_effort => {}
                            _ => best = Some((next, step_effort)),
                        }
                    }
                }
            }
            let (step_pos, step_effort) = best.unwrap();
            effort = effort.max(step_effort);
            pos = step_pos;
        }
        effort
    }
}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let finish = (heights.len() - 1, heights[0].len() - 1);
        let mut pos = (0, 0);
        let mut effort = 0;
        let mut visited = vec![vec![false; finish.1 + 1]; finish.0 + 1];
        let mut candidates = std::collections::BinaryHeap::new();
        while pos != finish {
            // Mark as visited
            visited[pos.0][pos.1] = true;

            // Update efforts for neighbors
            let height = heights[pos.0][pos.1];
            let mut update = |pos| {
                candidates.push(Candidate {
                    pos,
                    effort: (height - heights[pos.0][pos.1]).abs(),
                })
            };
            if pos.0 > 0 {
                update((pos.0 - 1, pos.1));
            }
            if pos.1 > 0 {
                update((pos.0, pos.1 - 1));
            }
            if pos.0 < finish.0 {
                update((pos.0 + 1, pos.1));
            }
            if pos.1 < finish.1 {
                update((pos.0, pos.1 + 1));
            }

            // Find the best next step
            loop {
                let next = candidates.pop().unwrap();
                if !visited[next.pos.0][next.pos.1] {
                    effort = effort.max(next.effort);
                    pos = next.pos;
                    break;
                }
            }
        }
        effort
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::minimum_effort_path(matrix![
                [1, 2, 2],
                [3, 8, 2],
                [5, 3, 5],
            ]),
            2,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::minimum_effort_path(matrix![
                [1, 2, 3],
                [3, 8, 4],
                [5, 3, 5],
            ]),
            1,
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::minimum_effort_path(matrix![
                [1, 2, 1, 1, 1],
                [1, 2, 1, 2, 1],
                [1, 2, 1, 2, 1],
                [1, 2, 1, 2, 1],
                [1, 1, 1, 2, 1],
            ]),
            0,
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_effort_path(matrix![[1000000]]), 0);
    }
}
