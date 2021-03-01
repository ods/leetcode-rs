// Broken Calculator
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3647/

struct Solution;

impl Solution {
    pub fn broken_calc(mut x: i32, y: i32) -> i32 {
        let mut muls = 0;
        while x < y {
            x <<= 1;
            muls += 1;
        }
        let diff = x - y;
        let max_dec = 2_i32.pow(muls as u32);
        muls + diff / max_dec + (diff % max_dec).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::broken_calc(2, 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::broken_calc(5, 8), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::broken_calc(3, 10), 3);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::broken_calc(1024, 1), 1023);
    }
}
