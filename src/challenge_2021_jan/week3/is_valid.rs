// Valid Parentheses
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3610/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            let open = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => {
                    stack.push(c);
                    continue;
                }
            };
            if stack.pop() != Some(open) {
                return false;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::is_valid("()".into()))
    }

    #[test]
    fn example2() {
        assert!(Solution::is_valid("()[]{}".into()))
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_valid("(]".into()))
    }

    #[test]
    fn example4() {
        assert!(!Solution::is_valid("([)]".into()))
    }

    #[test]
    fn example5() {
        assert!(Solution::is_valid("{[]}".into()))
    }
}
