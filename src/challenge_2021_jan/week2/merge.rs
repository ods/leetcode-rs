// Merge Sorted Array
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3600/

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx1 = m - 1;
        let mut idx2 = n - 1;
        for i in (0..(m + n) as usize).rev() {
            if idx2 == -1
                || (idx1 != -1 && nums1[idx1 as usize] > nums2[idx2 as usize])
            {
                nums1[i] = nums1[idx1 as usize];
                idx1 -= 1;
            } else {
                nums1[i] = nums2[idx2 as usize];
                idx2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        let mut arr1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut arr2: Vec<i32> = vec![2, 5, 6];
        super::Solution::merge(&mut arr1, 3, &mut arr2, 3);
        assert_eq!(arr1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example2() {
        let mut arr1: Vec<i32> = vec![0];
        let mut arr2: Vec<i32> = vec![1];
        super::Solution::merge(&mut arr1, 0, &mut arr2, 1);
        assert_eq!(arr1, vec![1]);
    }

    #[test]
    fn test_empty() {
        let mut arr1: Vec<i32> = vec![1];
        let mut arr2: Vec<i32> = vec![];
        super::Solution::merge(&mut arr1, 1, &mut arr2, 0);
        assert_eq!(arr1, vec![1]);
    }
}
