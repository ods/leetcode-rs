// Determine if Two Strings Are Close
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3613/

use std::collections::HashMap;

pub struct Solution;

#[derive(PartialEq, Eq)]
struct Signature {
    charset: Vec<char>,
    frequencies: Vec<usize>,
}

impl Signature {
    pub fn from(word: String) -> Self {
        let mut by_char: HashMap<char, usize> = HashMap::new();
        for char in word.chars() {
            *by_char.entry(char).or_default() += 1;
        }
        let mut charset: Vec<char> = by_char.keys().copied().collect();
        charset.sort_unstable();
        let mut frequencies: Vec<usize> = by_char.values().copied().collect();
        frequencies.sort_unstable();
        Self {
            charset,
            frequencies,
        }
    }
}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        word1.len() == word2.len()
            && Signature::from(word1) == Signature::from(word2)
    }

    // Top from leetcode
    #[cfg(disable)]
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut chars1 = [0; 26];
        for c in word1.bytes() {
            let idx = (c - b'a') as usize;
            chars1[idx] += 1;
        }

        let mut chars2 = [0; 26];
        for c in word2.bytes() {
            let idx = (c - b'a') as usize;
            if chars1[idx] == 0 {
                return false;
            }
            chars2[idx] += 1;
        }

        chars1.sort_unstable();
        chars2.sort_unstable();

        chars1 == chars2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(word1: &str, word2: &str, expected: bool) {
        assert_eq!(
            Solution::close_strings(word1.into(), word2.into()),
            expected
        );
    }

    #[test]
    fn example1() {
        check("abc", "bca", true);
    }

    #[test]
    fn example2() {
        check("a", "aa", false);
    }

    #[test]
    fn example3() {
        check("cabbba", "abbccc", true);
    }

    #[test]
    fn example4() {
        check("cabbba", "aabbss", false);
    }
}
