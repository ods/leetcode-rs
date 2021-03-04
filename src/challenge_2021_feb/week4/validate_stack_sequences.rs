// Validate Stack Sequences
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3653/

pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(
        pushed: Vec<i32>,
        popped: Vec<i32>,
    ) -> bool {
        let mut push_it = pushed.iter();
        let mut pop_it = popped.iter().peekable();
        let mut stack = Vec::new();
        while let Some(&val) = push_it.next() {
            stack.push(val);
            loop {
                match stack.last() {
                    Some(last) if pop_it.peek() == Some(&last) => {
                        stack.pop();
                        pop_it.next();
                    }
                    _ => break,
                }
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
        assert_eq!(
            Solution::validate_stack_sequences(
                vec![1, 2, 3, 4, 5],
                vec![4, 5, 3, 2, 1],
            ),
            true,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::validate_stack_sequences(
                vec![1, 2, 3, 4, 5],
                vec![4, 3, 5, 1, 2],
            ),
            false,
        );
    }
}
