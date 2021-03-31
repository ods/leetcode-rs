// Stamping The Sequence
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3691/

pub struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.as_bytes();
        let s_len = stamp.len();
        let mut target = target.into_bytes();
        let t_len = target.len();
        let mut res = Vec::new();
        let mut stamped_total = 0;

        while stamped_total < t_len {
            let mut was_stamped = false;
            'mid: for i in 0..=t_len - s_len {
                if res.contains(&(i as i32)) {
                    continue;
                }

                let mut stamped = 0;
                for (&sb, &tb) in stamp.iter().zip(target[i..].iter()) {
                    if tb == sb {
                        stamped += 1;
                    } else if tb != b'?' {
                        continue 'mid;
                    }
                }

                if stamped != 0 {
                    // There is no `fill` method in Leetcode's version
                    target[i..i + s_len].iter_mut().for_each(|b| *b = b'?');
                    was_stamped = true;
                    stamped_total += stamped;
                    res.push(i as i32);
                }
            }
            if !was_stamped {
                return Vec::new();
            }
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(stamp: &str, target: &str) {
        let result = Solution::moves_to_stamp(stamp.into(), target.into());
        assert!(result.len() <= 10 * target.len());
        let mut replay = vec![b'?'; target.len()];
        for start in result {
            let start = start as usize;
            replay[start..start + stamp.len()]
                .copy_from_slice(stamp.as_bytes());
        }
        assert_eq!(String::from_utf8(replay).unwrap(), target);
    }

    #[test]
    fn example1() {
        check("abc", "ababc");
    }

    #[test]
    fn example2() {
        check("abca", "aabcaca");
    }

    #[test]
    fn test_bad_case() {
        let stamp = "a".repeat(30) + "b";
        let pattern = "a".repeat(30) + "b";
        let target = stamp.clone() + &pattern.as_str().repeat(30);
        check(&stamp, &target);
    }
}
