// Maximum Frequency Stack
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3655/

use std::collections::HashMap;

pub struct FreqStack {
    stacks: Vec<Vec<i32>>,
    freqs: HashMap<i32, usize>,
}

impl FreqStack {
    pub fn new() -> Self {
        Self {
            stacks: Vec::new(),
            freqs: HashMap::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        let freq = self.freqs.entry(x).or_default();
        *freq += 1;
        if self.stacks.len() < *freq {
            self.stacks.push(Vec::new());
        }
        self.stacks[*freq - 1].push(x);
    }

    pub fn pop(&mut self) -> i32 {
        let last_stack = self.stacks.last_mut().unwrap();
        let x = last_stack.pop().unwrap();
        if last_stack.is_empty() {
            self.stacks.pop();
        }
        let freq = self.freqs.get_mut(&x).unwrap();
        *freq -= 1;
        if *freq == 0 {
            self.freqs.remove(&x);
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut stack = FreqStack::new();
        let mut res = Vec::new();
        for &item in &[5, 7, 5, 7, 4, 5] {
            stack.push(item);
        }
        for _ in 0..4 {
            res.push(stack.pop());
        }
        assert_eq!(res, vec![5, 7, 5, 4]);
    }
}
