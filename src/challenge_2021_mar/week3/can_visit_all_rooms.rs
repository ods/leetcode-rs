// Keys and Rooms
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3677/

pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = std::collections::HashSet::new();
        visited.insert(0);
        let mut key_pool = rooms[0].clone();
        while visited.len() < rooms.len() {
            match key_pool.pop() {
                None => return false,
                Some(key) => {
                    if visited.insert(key) {
                        for &new_key in &rooms[key as usize] {
                            if !visited.contains(&new_key) {
                                key_pool.push(new_key);
                            }
                        }
                    }
                }
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![
                vec![1],
                vec![2],
                vec![3],
                vec![]
            ]),
            true
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![
                vec![1, 3],
                vec![3, 0, 1],
                vec![2],
                vec![0]
            ]),
            false
        );
    }
}
