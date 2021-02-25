// Boats to Save People
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3602/

pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people.clone();
        people.sort_unstable();
        let mut low = 0;
        let mut high = people.len();
        let mut res = 0;
        while high > low {
            high -= 1;
            if low != high && people[high] + people[low] <= limit {
                low += 1;
            }
            res += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
