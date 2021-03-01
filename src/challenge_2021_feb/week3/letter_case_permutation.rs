// Letter Case Permutation
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3642/

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res = vec![String::new()];
        for c in s.chars() {
            let to_extend: Vec<String> = match c {
                'a'..='z' => {
                    let tail = c.to_uppercase().to_string();
                    res.iter().cloned().map(|s| s + &tail).collect()
                }
                'A'..='Z' => {
                    let tail = c.to_lowercase().to_string();
                    res.iter().cloned().map(|s| s + &tail).collect()
                }
                _ => Vec::new(),
            };
            res.iter_mut().for_each(|s| s.push(c));
            res.extend(to_extend);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, expected: &[&str]) {
        let mut expected = expected
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<String>>();
        expected.sort_unstable();
        let mut res = Solution::letter_case_permutation(String::from(s));
        res.sort_unstable();
        assert_eq!(res, expected);
    }

    #[test]
    fn example1() {
        check("a1b2", &["a1b2", "a1B2", "A1b2", "A1B2"]);
    }

    #[test]
    fn example2() {
        check("3z4", &["3z4", "3Z4"]);
    }

    #[test]
    fn example3() {
        check("12345", &["12345"]);
    }

    #[test]
    fn example4() {
        check("0", &["0"]);
    }
}
