// Stamping The Sequence
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3691/

pub struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.as_bytes();
        let s_len = stamp.len();
        let target = target.as_bytes();
        let t_len = target.len();
        // Positions of letters in stamp
        let mut s_ps = vec![Vec::<usize>::new(); (b'z' - b'a' + 1) as usize];
        for (i, &b) in stamp.iter().enumerate() {
            s_ps[(b - b'a') as usize].push(i);
        }

        let mut till_end = Vec::<i32>::new(); // Apply at the end in reverse order
        let mut res = Vec::new();

        let mut from_start = true; // Matching from start is required?
        let mut pos = 0;

        while pos < t_len {
            let mut m_len = 0;
            if from_start {
                while m_len < s_len
                    && pos + m_len < t_len
                    && target[pos + m_len] == stamp[m_len]
                {
                    m_len += 1;
                }
                if m_len == 0 {
                    return Vec::new();
                }
                if m_len == s_len {
                    till_end.push(pos as i32);
                    from_start = false;
                } else {
                    res.push(pos as i32);
                    from_start = true;
                }
            } else {
                let b = target[pos + m_len];
                let mut ps = s_ps[(b - b'a') as usize].clone();
                if ps.is_empty() {
                    return Vec::new();
                }
                m_len += 1;
                while pos + m_len < t_len {
                    let mut next_ps = Vec::<usize>::new();
                    let b = target[pos + m_len];
                    for &p in &ps {
                        if p < s_len - 1 && b == stamp[p + 1] {
                            next_ps.extend(&s_ps[(b - b'a') as usize]);
                        }
                    }
                    if next_ps.is_empty() {
                        break;
                    }
                    m_len += 1;
                    ps = next_ps;
                }
                let &s_p = ps.last().unwrap();
                let start = (pos + m_len - s_p - 1) as i32;
                if s_p == s_len - 1 {
                    till_end.push(start);
                } else {
                    res.push(start)
                }
                from_start = s_p != s_len - 1;
            }
            pos += m_len;
        }
        if from_start {
            // The last match wan't till the end
            return Vec::new();
        }

        till_end.reverse();
        res.extend(till_end);
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
        let pattern = "a".repeat(5) + "b";
        let target = stamp.clone() + &pattern.as_str().repeat(5);
        check(&stamp, &target);
    }
}
