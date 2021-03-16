// Best Time to Buy and Sell Stock with Transaction Fee
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3674/

pub struct Solution;

enum State {
    Min(i32),
    MinMax(i32, i32),
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut profit = 0;
        let mut state = State::Min(prices[0]);
        for &price in prices[1..].iter() {
            match state {
                State::Min(min) => {
                    if price < min {
                        state = State::Min(price);
                    } else if price > min {
                        state = State::MinMax(min, price);
                    }
                }
                State::MinMax(min, max) => {
                    if price > max {
                        state = State::MinMax(min, price);
                    } else if price < min.max(max - fee) {
                        if max - min > fee {
                            profit += max - min - fee;
                        }
                        state = State::Min(price);
                    }
                }
            }
        }
        if let State::MinMax(min, max) = state {
            if max - min > fee {
                profit += max - min - fee;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }

    #[test]
    fn test_desc() {
        assert_eq!(Solution::max_profit(vec![4, 3, 4, 2, 3, 1], 1), 0);
    }
}
