// Check If All 1's Are at Least Length K Places Away
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3616/

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        nums.iter()
            .try_fold(0, |forbid, &item| match (forbid, item) {
                (0, 0) => Result::Ok(0),
                (0, 1) => Result::Ok(k),
                (_, 0) => Result::Ok(forbid - 1),
                (_, 1) => Result::Err(()),
                _ => unreachable!(),
            })
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(arr: &[i32], k: i32, expected: bool) {
        assert_eq!(Solution::k_length_apart(arr.into(), k), expected);
    }

    #[test]
    fn example1() {
        check(&[1, 0, 0, 0, 1, 0, 0, 1], 2, true);
    }

    #[test]
    fn example2() {
        check(&[1, 0, 0, 1, 0, 1], 2, false);
    }

    #[test]
    fn example3() {
        check(&[1, 1, 1, 1, 1], 0, true);
    }

    #[test]
    fn example4() {
        check(&[0, 1, 0, 1], 1, true);
    }
}
