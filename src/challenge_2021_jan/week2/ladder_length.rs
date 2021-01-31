// Word Ladder
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3598/

struct Solution;

impl Solution {
    pub fn ladder_length(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        let begin_word: &str = &begin_word;
        let end_word: &str = &end_word;
        let mut word_list: Vec<_> = word_list
            .iter()
            .map(|s| &**s)
            .filter(|&w| w != begin_word)
            .collect();
        let mut candidates: Vec<&str> = vec![&begin_word];
        for gen in 1.. {
            if candidates.contains(&end_word) {
                return gen;
            }
            let partitions: (Vec<&str>, Vec<&str>) = word_list
                .iter()
                .partition(|w| candidates.iter().any(|c| is_close(w, c)));

            candidates = partitions.0;
            word_list = partitions.1;
            if candidates.is_empty() {
                break;
            }
        }
        0
    }
}

fn is_close(word1: &str, word2: &str) -> bool {
    match word1
        .chars()
        .zip(word2.chars())
        .try_fold(0, |acc, (c1, c2)| {
            let res = acc + (c1 != c2) as i32;
            if res > 1 {
                None
            } else {
                Some(res)
            }
        }) {
        None => false,
        Some(1) => true,
        _ => unreachable!(),
    }
}

mod test {
    use super::*;

    #[test]
    fn is_close1() {
        assert_eq!(is_close("hit", "hot"), true);
    }

    #[test]
    fn is_close2() {
        assert_eq!(is_close("hit", "cog"), false);
    }

    #[test]
    fn test_length_eq() {
        assert_eq!(
            Solution::ladder_length(
                "hit".into(),
                "hit".into(),
                vec!["hot".into(), "dot".into()],
            ),
            1,
        );
    }

    #[test]
    fn test_length_dup() {
        assert_eq!(
            Solution::ladder_length(
                "hit".into(),
                "dot".into(),
                vec![
                    "hit".into(),
                    "hot".into(),
                    "hot".into(),
                    "dot".into(),
                    "dot".into()
                ],
            ),
            3,
        );
    }

    #[test]
    fn test0() {
        assert_eq!(
            Solution::ladder_length(
                "hit".into(),
                "dot".into(),
                vec!["hot".into(), "dot".into()],
            ),
            3,
        );
    }

    #[test]
    fn example1() {
        assert_eq!(
            Solution::ladder_length(
                "hit".into(),
                "cog".into(),
                vec![
                    "hot".into(),
                    "dot".into(),
                    "dog".into(),
                    "lot".into(),
                    "log".into(),
                    "cog".into(),
                ],
            ),
            5,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::ladder_length(
                "hit".into(),
                "cog".into(),
                vec![
                    "hot".into(),
                    "dot".into(),
                    "dog".into(),
                    "lot".into(),
                    "log".into(),
                ],
            ),
            0,
        );
    }

    #[test]
    fn test_loop() {
        Solution::ladder_length(
            "qa".into(),
            "sq".into(),
            [
                "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb",
                "kr", "ln", "tm", "le", "av", "sm", "ar", "ci", "ca", "br",
                "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr",
                "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh",
                "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb",
                "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz",
                "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an",
                "me", "mo", "na", "la", "st", "er", "sc", "ne", "mn", "mi",
                "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr",
                "pa", "he", "lr", "sq", "ye",
            ]
            .iter()
            .cloned()
            .map(String::from)
            .collect(),
        );
    }
}
