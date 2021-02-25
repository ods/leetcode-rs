// Count Sorted Vowel Strings
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3607/

struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        const NUM_VOWELS: usize = 5;
        let mut by_1st_char = [1; NUM_VOWELS];
        for _ in 2..=n {
            let mut sum = 1;
            for vowel in 1..NUM_VOWELS {
                let num = unsafe { by_1st_char.get_unchecked_mut(vowel) };
                sum += *num;
                *num = sum;
            }
        }
        by_1st_char.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
