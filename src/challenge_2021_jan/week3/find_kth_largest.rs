// Kth Largest Element in an Array
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3606/

#[cfg(disabled)]
use std::cmp::Ordering::*;

struct Solution;

impl Solution {
    #[cfg(disable)]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut buf = Vec::with_capacity(k);
        for num in nums {
            let filled = buf.len() >= k;
            if filled && num <= buf[0] {
                continue;
            };
            let pos = buf
                .binary_search(&num)
                .unwrap_or_else(std::convert::identity);
            if filled {
                buf[..pos].rotate_left(1);
                buf[pos - 1] = num;
            } else {
                buf.insert(pos, num);
            }
        }
        buf[0]
    }

    #[cfg(disable)]
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let size = nums.len();
        let (left, right) = nums.split_at_mut(size - k);
        right.sort_unstable();
        for &num in left.iter() {
            if num > right[0] {
                let pos = right
                    .binary_search(&num)
                    .unwrap_or_else(std::convert::identity);
                right[..pos].rotate_left(1);
                right[pos - 1] = num;
            }
        }
        right[0]
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let size = nums.len();
        let (left, right) = nums.split_at_mut(size - k);
        right.sort_unstable();
        for &num in left.iter() {
            if num > right[0] {
                let pos = right
                    .binary_search(&num)
                    .unwrap_or_else(std::convert::identity);
                right.copy_within(1..pos, 0);
                right[pos - 1] = num;
            }
        }
        right[0]
    }

    #[cfg(disable)]
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let idx = nums.len() - k as usize;
        nums.select_nth_unstable(idx);
        nums[idx]
    }

    #[cfg(disable)]
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let idx = nums.len() - k as usize;
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let pivot = nums[high];
            let mut store = low;
            for i in low..high {
                if nums[i] < pivot {
                    nums.swap(i, store);
                    store += 1;
                }
            }
            nums.swap(store, high);
            match store.cmp(&idx) {
                Equal => break,
                Less => low = store + 1,
                Greater => high = store - 1,
            }
        }
        if nums[low] > nums[high] {
            nums.swap(low, high)
        };
        nums[idx]
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5)
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        )
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::find_kth_largest(vec![99, 99], 1), 99)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4], 3), 2)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::find_kth_largest(vec![1, 2], 1), 2)
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5)
    }
}
