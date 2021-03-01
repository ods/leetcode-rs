// Valid Anagram
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3636/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut by_char: HashMap<char, usize> = HashMap::new();
        for char in s.chars() {
            *by_char.entry(char).or_default() += 1;
        }
        for char in t.chars() {
            let entry = by_char.entry(char).or_default();
            if *entry == 0 {
                return false;
            } else {
                *entry -= 1;
            }
        }
        true
    }
}

#[cfg(disable)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut sv = s.chars().collect::<Vec<_>>();
        let mut tv = t.chars().collect::<Vec<_>>();
        sv.sort_unstable();
        tv.sort_unstable();
        sv == tv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, t: &str, res: bool) {
        assert_eq!(
            Solution::is_anagram(String::from(s), String::from(t)),
            res
        );
    }

    #[test]
    fn example1() {
        check("anagram", "nagaram", true);
    }

    #[test]
    fn example2() {
        check("rat", "car", false);
    }
}
