// Remove Palindromic Subsequences
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3665/

pub struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mid = s.len() / 2;
        if s[..mid].chars().eq(s[s.len() - mid..].chars().rev()) {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::remove_palindrome_sub("".to_string()), 0);
    }
}
