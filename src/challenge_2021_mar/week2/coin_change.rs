// Coin Change
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3668/

pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let mut counts = vec![-1; amount as usize];
        for coin in coins {
            if coin > amount {
                break;
            }
            for idx in 0..counts.len() {
                let val = (idx + 1) as i32;
                match coin.cmp(&val) {
                    Equal => {
                        counts[idx] = 1;
                    }
                    Less => {
                        let prev = counts[(val - coin - 1) as usize];
                        if prev != -1 {
                            if counts[idx] == -1 || counts[idx] > prev + 1 {
                                counts[idx] = prev + 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        *counts.last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }
}
