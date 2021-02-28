// Maximum Frequency Stack
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3655/

use std::collections::{BinaryHeap, HashMap};

struct FreqStack {
    heap: BinaryHeap<(usize, usize, i32)>,
    freqs: HashMap<i32, usize>,
    gen: usize,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            freqs: HashMap::new(),
            gen: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let freq = self.freqs.entry(x).or_default();
        self.heap.push((*freq, self.gen, x));
        *freq += 1;
        self.gen += 1;
    }

    fn pop(&mut self) -> i32 {
        let (_, _, x) = self.heap.pop().unwrap();
        let freq = self.freqs.get_mut(&x).unwrap();
        *freq -= 1;
        if *freq == 0 {
            self.freqs.remove(&x);
        }
        x
    }
}

#[cfg(test)]
mod test {
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
