// Divide Two Integers
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3654/

struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let negative = dividend.is_negative() ^ divisor.is_negative();
        let mut num = (dividend as i64).abs();
        let den = (divisor as i64).abs();

        let mut mult2 = 0;
        while den << mult2 < num {
            mult2 += 1;
        }

        let mut res = 0_i64;
        while mult2 >= 0 {
            res <<= 1;
            let prod = den << mult2;
            if num >= prod {
                num -= prod;
                res += 1;
            }
            mult2 -= 1;
        }

        (if negative { -res } else { res })
            .try_into()
            .unwrap_or(std::i32::MAX)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(dividend: i32, divisor: i32) {
        let expected = dividend.checked_div(divisor).unwrap_or(std::i32::MAX);
        assert_eq!(Solution::divide(dividend, divisor), expected);
    }

    #[test]
    fn example1() {
        check(10, 3);
    }

    #[test]
    fn example2() {
        check(7, -3);
    }

    #[test]
    fn example3() {
        check(0, 1);
    }

    #[test]
    fn example4() {
        check(1, 1);
    }

    #[test]
    fn test_big() {
        check(12345678, 123);
    }

    #[test]
    fn test_overflow() {
        check(-2147483648, -1);
    }

    #[test]
    fn test_no_overflow1() {
        check(-2147483648, 1);
    }

    #[test]
    fn test_no_overflow2() {
        check(-2147483648, -2147483648);
    }

    #[test]
    fn test_no_overflow3() {
        check(-2147483648, 2147483647);
    }
}
