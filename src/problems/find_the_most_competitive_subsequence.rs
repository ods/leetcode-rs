// 1673. Find the Most Competitive Subsequence
// https://leetcode.com/problems/find-the-most-competitive-subsequence/

struct Solution;

impl Solution {
    // Causes timeout
    #[cfg(disable)]
    pub fn most_competitive(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut start = 0;
        while start < k {
            let mut it =
                nums[start..=nums.len() - k + start].iter().enumerate();
            let first = it.next().unwrap();
            let min_idx = it
                .fold(
                    first,
                    |(mi, m), (i, v)| {
                        if v < m {
                            (i, v)
                        } else {
                            (mi, m)
                        }
                    },
                )
                .0;
            nums.drain(start..start + min_idx);
            start += 1;
        }
        nums.drain(start..);
        nums
    }

    // 1st accepted (in-place)
    #[cfg(disable)]
    pub fn most_competitive(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut start = 0;
        while nums.len() > k && start < nums.len() - 1 {
            if nums[start] > nums[start + 1] {
                nums.remove(start);
                if start > 0 {
                    start -= 1;
                }
            } else {
                start += 1;
            }
        }
        nums.drain(k..);
        nums
    }

    // Faster
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut needed = k as usize;
        let mut left = nums.len();
        let mut res = Vec::with_capacity(needed);
        for num in nums {
            // Would be more readable, but is not supported in  Leetcode's
            // version:
            // matches!(res.last(), Some(&last) if last > num)
            while left > needed && res.last() > Some(&num) {
                res.pop();
                needed += 1;
            }
            if needed > 0 {
                needed -= 1;
                res.push(num)
            }
            left -= 1;
        }
        res
    }

    // One more, but not better
    #[cfg(disable)]
    fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut arr = Vec::new();
        let mut m = 0;
        for i in 0..n {
            while let Some(&top) = arr.last() {
                if top > nums[i] && k < n - m {
                    m += 1;
                    arr.pop();
                } else {
                    break;
                }
            }
            arr.push(nums[i]);
        }
        arr[0..k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::most_competitive(vec![3, 5, 2, 6], 2),
            vec![2, 6],
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4),
            vec![2, 3, 3, 4],
        );
    }

    #[test]
    fn test_desc() {
        assert_eq!(
            Solution::most_competitive(vec![3, 2, 1], 3),
            vec![3, 2, 1],
        );
    }

    #[test]
    fn test_tail() {
        assert_eq!(
            Solution::most_competitive(
                vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2],
                3,
            ),
            vec![8, 80, 2],
        );
    }

    #[test]
    fn test_big() {
        let mut src = vec![1; 100_000];
        *src.last_mut().unwrap() = 0;
        let mut res = vec![1; 5_000];
        *res.last_mut().unwrap() = 0;
        assert_eq!(Solution::most_competitive(src, 5_000), res);
    }
}
