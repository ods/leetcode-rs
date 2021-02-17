// Container With Most Water
// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3643/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = height.len() - 1;
        let mut max_area = 0;
        while low < high {
            max_area = max_area
                .max(height[low].min(height[high]) * (high - low) as i32);
            if height[low] < height[high] {
                low += 1;
            } else {
                high -= 1;
            }
        }
        max_area
    }
}

mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
