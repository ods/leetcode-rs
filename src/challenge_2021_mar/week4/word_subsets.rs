// Word Subsets
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3685/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn byte_counts(s: &str) -> HashMap<u8, usize> {
        let mut counts = HashMap::new();
        s.bytes().for_each(|b| *counts.entry(b).or_default() += 1);
        counts
    }

    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut b_max_counts = HashMap::<u8, usize>::new();
        for sb in b {
            for (bb, count) in Self::byte_counts(&sb) {
                b_max_counts
                    .entry(bb)
                    .and_modify(|e| *e = (*e).max(count))
                    .or_insert(count);
            }
        }

        let mut res = Vec::new();
        'a: for sa in a {
            let a_counts = Self::byte_counts(&sa);
            for (bb, &b_count) in &b_max_counts {
                match a_counts.get(bb) {
                    Some(&a_count) if a_count >= b_count => {}
                    _ => continue 'a,
                }
            }
            res.push(sa);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_vec;

    macro_rules! a {
        () => {
            string_vec!["amazon", "apple", "facebook", "google", "leetcode"]
        };
    }

    #[test]
    fn example1() {
        assert_eq!(
            Solution::word_subsets(a!(), string_vec!["e", "o"]),
            string_vec!["facebook", "google", "leetcode"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::word_subsets(a!(), string_vec!["l", "e"]),
            string_vec!["apple", "google", "leetcode"]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::word_subsets(a!(), string_vec!["e", "oo"]),
            string_vec!["facebook", "google"]
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::word_subsets(a!(), string_vec!["lo", "eo"]),
            string_vec!["google", "leetcode"]
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::word_subsets(a!(), string_vec!["ec", "oc", "ceo"]),
            string_vec!["facebook", "leetcode"]
        );
    }
}
