// Short Encoding of Words
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3662/

pub struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut rev_words: Vec<String> = words
            .iter()
            .map(|word| word.chars().rev().collect())
            .collect();
        rev_words.sort_unstable();
        let mut prev = "";
        let mut res = 0;
        for cur in rev_words.iter() {
            if !cur.starts_with(prev) {
                res += prev.len() + 1;
            }
            prev = cur.as_str();
        }
        if !prev.is_empty() {
            res += prev.len() + 1;
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(words: &[&str], expected: i32) {
        let vec = words.iter().copied().map(String::from).collect();
        assert_eq!(Solution::minimum_length_encoding(vec), expected);
    }

    #[test]
    fn example1() {
        check(&["time", "me", "bell"], 10);
    }

    #[test]
    fn example2() {
        check(&["t"], 2);
    }
}
