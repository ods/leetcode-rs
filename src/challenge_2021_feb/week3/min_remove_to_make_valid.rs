// Minimum Remove to Make Valid Parentheses
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3645/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut openned = Vec::new();
        let mut to_drop = HashSet::new();
        for (idx, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    openned.push(idx);
                }
                ')' => {
                    if openned.pop().is_none() {
                        to_drop.insert(idx);
                    }
                }
                _ => {}
            }
        }
        while let Some(idx) = openned.pop() {
            to_drop.insert(idx);
        }
        s.chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                if to_drop.contains(&idx) {
                    None
                } else {
                    Some(c)
                }
            })
            .collect()
    }
}

mod test {
    use super::*;

    fn is_valid(s: &str) -> bool {
        let mut openned = 0;
        for c in s.chars() {
            match c {
                ')' => {
                    if openned == 0 {
                        return false;
                    }
                    openned -= 1;
                }
                '(' => {
                    openned += 1;
                }
                _ => {}
            };
        }
        openned == 0
    }

    fn check(s: &str, removed: usize) {
        let res = Solution::min_remove_to_make_valid(String::from(s));
        assert!(is_valid(&res));
        assert_eq!(res.len(), s.len() - removed);
    }

    #[test]
    fn example1() {
        check("lee(t(c)o)de)", 1);
    }

    #[test]
    fn example2() {
        check("a)b(c)d", 1);
    }

    #[test]
    fn example3() {
        check("))((", 4);
    }

    #[test]
    fn example4() {
        check("(a(b(c)d)", 1);
    }

    #[test]
    fn test_overlap() {
        check("a)b(c(d)e", 2);
    }
}
