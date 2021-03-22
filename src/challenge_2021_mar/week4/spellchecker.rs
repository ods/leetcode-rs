// Vowel Spellchecker
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3681/

pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn vowel_pattern(word_lower: &str) -> String {
        word_lower
            .chars()
            .map(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '.',
                _ => c,
            })
            .collect::<String>()
    }

    pub fn spellchecker(
        wordlist: Vec<String>,
        queries: Vec<String>,
    ) -> Vec<String> {
        let mut exact = HashSet::<&str>::new();
        let mut lower = HashMap::<String, &str>::new();
        let mut vowel = HashMap::<String, &str>::new();
        for word in wordlist.iter().rev() {
            exact.insert(word);
            let word_lower = word.to_ascii_lowercase();
            let word_vowel = Self::vowel_pattern(&word_lower);
            lower.insert(word_lower, word);
            vowel.insert(word_vowel, word);
        }

        let mut res = Vec::with_capacity(queries.len());
        for query in queries {
            if exact.contains(query.as_str()) {
                res.push(query);
                continue;
            }
            let query_lower = query.to_ascii_lowercase();
            if let Some(word) = lower.get(&query_lower) {
                res.push(word.to_string());
                continue;
            }
            let query_vowel = Self::vowel_pattern(&query_lower);
            if let Some(word) = vowel.get(&query_vowel) {
                res.push(word.to_string());
                continue;
            }
            res.push(String::new());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_vec;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::spellchecker(
                string_vec!["KiTe", "kite", "hare", "Hare"],
                string_vec![
                    "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear",
                    "keti", "keet", "keto"
                ]
            ),
            string_vec![
                "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "",
                "KiTe"
            ]
        )
    }
}
