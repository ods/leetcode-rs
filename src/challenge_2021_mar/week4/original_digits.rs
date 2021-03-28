// Reconstruct Original Digits from English
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3687/

pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut counts = [0; (b'z' - b'e') as usize + 1];
        for b in s.as_bytes() {
            counts[(b - b'e') as usize] += 1;
        }
        let count = |b| counts[(b - b'e') as usize];

        let mut res = String::with_capacity(s.len() / 3);

        (0..count(b'z')).for_each(|_| res.push('0'));
        let count_1 = count(b'o') - count(b'z') - count(b'w') - count(b'u');
        (0..count_1).for_each(|_| res.push('1'));
        (0..count(b'w')).for_each(|_| res.push('2'));
        (0..count(b'h') - count(b'g')).for_each(|_| res.push('3'));
        (0..count(b'u')).for_each(|_| res.push('4'));
        (0..count(b'f') - count(b'u')).for_each(|_| res.push('5'));
        (0..count(b'x')).for_each(|_| res.push('6'));
        let count_7 = count(b's') - count(b'x');
        (0..count_7).for_each(|_| res.push('7'));
        (0..count(b'g')).for_each(|_| res.push('8'));
        let count_9 = (count(b'n') - count_1 - count_7) / 2;
        (0..count_9).for_each(|_| res.push('9'));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::original_digits("owoztneoer".into()), "012");
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::original_digits("fviefuro".into()), "45");
    }

    #[test]
    fn test_all_once() {
        assert_eq!(
            Solution::original_digits(
                "zeroonetwothreefourfivesixseveneightnine".into()
            ),
            "0123456789"
        );
    }
}
