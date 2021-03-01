// 1649. Create Sorted Array through Instructions
// https://leetcode.com/problems/create-sorted-array-through-instructions/

struct Solution;

const MODULO: i32 = 1_000_000_007;

#[cfg(disable)]
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut res = Vec::with_capacity(instructions.len());
        let mut cost: i32 = 0;
        for inst in instructions {
            let value = inst * 2;
            let lower = res.binary_search(&(value - 1)).unwrap_err();
            let upper = res.binary_search(&(value + 1)).unwrap_err();
            cost += lower.min(res.len() - upper) as i32;
            res.insert(upper, value);

            if cost > MODULO {
                cost %= MODULO;
            }
        }
        cost
    }
}

// Raybko (a.k.a. Fenwick) tree
struct RyabkoTree(Vec<usize>);

impl RyabkoTree {
    fn new(max: usize) -> Self {
        Self(vec![0; max + 1])
    }
    fn add(&mut self, mut num: usize) {
        while num < self.0.len() {
            self.0[num] += 1;
            num += num - (num & (num - 1));
        }
    }
    fn get(&self, mut num: usize) -> usize {
        let mut sum = 0;
        loop {
            sum += self.0[num];
            if num == 0 {
                break sum;
            }
            let last_bit = num - (num & (num - 1));
            if num < last_bit {
                break sum;
            }
            num -= last_bit;
        }
    }
}

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut tree =
            RyabkoTree::new(*instructions.iter().max().unwrap_or(&0) as usize);
        let mut cost = 0;
        for (count, &num) in instructions.iter().enumerate() {
            let lower = tree.get((num - 1) as usize);
            let upper = count - tree.get(num as usize);
            cost = (cost + lower.min(upper) as i32) % MODULO;
            tree.add(num as usize);
        }
        cost as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3)
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]),
            4
        )
    }

    #[test]
    fn test_big() {
        let instructions: Vec<i32> =
            [4, 5, 6].iter().cycle().take(90_000).copied().collect();
        Solution::create_sorted_array(instructions);
    }
}
