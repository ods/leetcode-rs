// Get Maximum in Generated Array
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3605/

struct Solution;

const SIZE: usize = 101;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums: [i32; SIZE] = [0; SIZE];
        nums[1] = 1;
        if n < 2 {
            return nums[n as usize];
        }
        let mut max = 1;
        for i in 2..=(n as usize) {
            let num = if i % 2 == 0 {
                nums[i / 2]
            } else {
                nums[i / 2] + nums[i / 2 + 1]
            };
            nums[i] = num;
            max = max.max(num);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_maximum_generated(2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}
