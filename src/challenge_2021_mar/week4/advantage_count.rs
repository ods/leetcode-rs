// Advantage Shuffle
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3683/

pub struct Solution;

impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        assert_eq!(a.len(), b.len());
        a.sort_unstable();
        let mut start = 0;
        let mut end = a.len();
        let mut b_idx = b
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val, idx))
            .collect::<Vec<_>>();
        b_idx.sort_unstable();
        let mut res = vec![0; a.len()];
        for (val, idx) in b_idx.into_iter().rev() {
            if a[end - 1] > val {
                res[idx] = a[end - 1];
                end -= 1;
            } else {
                res[idx] = a[start];
                start += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::advantage_count(
                vec![12, 24, 8, 32],
                vec![13, 25, 32, 11]
            ),
            vec![24, 32, 8, 12]
        );
    }
}
