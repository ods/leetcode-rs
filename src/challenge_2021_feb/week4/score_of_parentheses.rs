// Score of Parentheses
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3651/

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for c in s.chars() {
            match c {
                '(' => stack.push(0),
                ')' => {
                    let val = match stack.pop().unwrap() {
                        0 => 1,
                        val => val * 2,
                    };
                    *stack.last_mut().unwrap() += val;
                }
                _ => unreachable!(),
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::score_of_parentheses("()".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::score_of_parentheses("(())".into()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::score_of_parentheses("()()".into()), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::score_of_parentheses("(()(()))".into()), 6);
    }
}
