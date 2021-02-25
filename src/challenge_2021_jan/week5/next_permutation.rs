// Next Permutation
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3623/

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 {
            return;
        }

        // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
        // Find the largest index k such that a[k] < a[k + 1]
        let k = match (0..n - 1).rfind(|&i| nums[i] < nums[i + 1]) {
            None => {
                // If no such index exists, the permutation is the last
                // permutation
                nums.reverse();
                return;
            }
            Some(i) => i,
        };
        // Find the largest index l greater than k such that a[k] < a[l]
        let l = (k + 1..n).rfind(|&i| nums[k] < nums[i]).unwrap();

        // Swap the value of a[k] with that of a[l]
        nums.swap(k, l);

        // Reverse the sequence from a[k + 1] up to and including the final
        // element a[n]
        nums[k + 1..].reverse();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn example4() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn fail1() {
        let mut nums = vec![1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }
}
