// Shortest Distance to a Character
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3631/

pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let max_dist = s.len() as i32;
        let mut res = Vec::with_capacity(s.len());
        let mut cur_dist = max_dist;
        for (idx, cur_c) in s.chars().enumerate() {
            if cur_c == c {
                cur_dist = 0;
                let mut back_dist = 1;
                for back_idx in (0..idx).rev() {
                    if res[back_idx] <= back_dist {
                        break;
                    }
                    res[back_idx] = back_dist;
                    back_dist += 1;
                }
            } else {
                cur_dist = (cur_dist + 1).min(max_dist);
            }
            res.push(cur_dist);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, c: char, expected: &[i32]) {
        assert_eq!(
            Solution::shortest_to_char(String::from(s), c),
            expected.to_vec()
        )
    }

    #[test]
    fn example1() {
        check("loveleetcode", 'e', &[3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
    }

    #[test]
    fn example2() {
        check("aaab", 'b', &[3, 2, 1, 0]);
    }

    #[test]
    fn test1() {
        check(&"a".repeat(10000), 'a', &[0; 10000]);
    }
}
