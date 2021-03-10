// Integer to Roman
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3667/

pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut res = String::new();
        for (val, repr) in &[
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ] {
            while num >= *val {
                res.push_str(repr);
                num -= val;
            }
        }
        assert_eq!(num, 0);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
