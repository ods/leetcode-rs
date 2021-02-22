// Longest Word in Dictionary through Deleting
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3649/

struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        let max_len = s.len();
        d.retain(|ds| ds.len() <= max_len);
        d.sort_unstable_by(|s1, s2| s2.len().cmp(&s1.len()).then(s1.cmp(s2)));
        d.into_iter()
            .find(|cand| {
                let mut it = s.chars();
                cand.chars()
                    // Find 1st candidate char not in `s`
                    .find(|cand_c| it.find(|sc| sc == cand_c).is_none())
                    .is_none()
            })
            .unwrap_or_default()
    }
}

mod test {
    use super::*;

    fn check(s: &str, d: Vec<&str>, expected: &str) {
        assert_eq!(
            Solution::find_longest_word(
                String::from(s),
                d.into_iter().map(String::from).collect()
            ),
            String::from(expected)
        )
    }

    #[test]
    fn example1() {
        check("abpcplea", vec!["ale", "apple", "monkey", "plea"], "apple");
    }

    #[test]
    fn example2() {
        check("abpcplea", vec!["a", "b", "c"], "a");
    }

    #[test]
    fn test_no_match() {
        check("bpcplex", vec!["ale", "apple", "monkey", "plea"], "");
    }
}
