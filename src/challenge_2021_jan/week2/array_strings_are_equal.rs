// Check If Two String Arrays are Equivalent
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3597/

pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(
        word1: Vec<String>,
        word2: Vec<String>,
    ) -> bool {
        word1
            .iter()
            .flat_map(|s| s.bytes())
            .eq(word2.iter().flat_map(|s| s.bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(word1: &[&str], word2: &[&str], expected: bool) {
        assert_eq!(
            Solution::array_strings_are_equal(
                word1.iter().copied().map(String::from).collect(),
                word2.iter().copied().map(String::from).collect(),
            ),
            expected,
        );
    }

    #[test]
    fn example1() {
        check(&["ab", "c"], &["a", "bc"], true);
    }

    #[test]
    fn example2() {
        check(&["a", "cb"], &["ab", "c"], false);
    }

    #[test]
    fn example3() {
        check(&["abc", "d", "defg"], &["abcddefg"], true);
    }
}
