// Binary Trees With Factors
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3670/

pub struct Solution;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        const MODULO: u64 = 1_000_000_007;
        arr.sort_unstable();
        let mut counts = std::collections::HashMap::new();
        let mut total = 0;
        for &num in arr.iter() {
            let mut count = 1_u64;
            for &factor1 in arr.iter() {
                if factor1 as u64 * factor1 as u64 > num as u64 {
                    break;
                }
                if num % factor1 != 0 {
                    continue;
                }
                let factor2 = num / factor1;
                if let Some(count2) = counts.get(&factor2) {
                    count = (count
                        + counts[&factor1]
                            * count2
                            * if factor1 == factor2 { 1 } else { 2 })
                        % MODULO;
                }
            }
            counts.insert(num, count);
            total = (total + count) % MODULO;
        }
        total as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::num_factored_binary_trees(vec![18, 3, 6, 2]), 12);
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::num_factored_binary_trees(vec![
                45, 42, 2, 18, 23, 1170, 12, 41, 40, 9, 47, 24, 33, 28, 10,
                32, 29, 17, 46, 11, 759, 37, 6, 26, 21, 49, 31, 14, 19, 8, 13,
                7, 27, 22, 3, 36, 34, 38, 39, 30, 43, 15, 4, 16, 35, 25, 20,
                44, 5, 48
            ]),
            777
        );
    }

    #[test]
    fn test_big_dont_panic() {
        let arr = (1..10_000).map(|val| val * 100_000).collect();
        assert_eq!(Solution::num_factored_binary_trees(arr), 9_999);
    }
}
