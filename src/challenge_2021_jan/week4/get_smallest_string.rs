// Smallest String With A Given Numeric Value
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3619/

struct Solution;

// First solution
#[cfg(disable)]
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let num_of_z = (k - n) / 25;
        let inter_add = (k - n) % 25;
        let num_of_a = n - num_of_z - (inter_add != 0) as usize;
        let mut res = String::with_capacity(n);
        for _ in 0..num_of_a {
            res.push('a');
        }
        if inter_add != 0 {
            res.push(('a' as u8 + inter_add as u8) as char);
        }
        for _ in 0..num_of_z {
            res.push('z');
        }
        res
    }
}

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut bytes = vec![0u8; n as _];
        let mut needed = k - n;

        for c in bytes.iter_mut().rev() {
            let over = needed.min(25);
            *c = b'a' + over as u8;
            needed -= over;
        }

        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

mod test {
    use super::*;

    #[test]
    fn test0a() {
        assert_eq!(&Solution::get_smallest_string(1, 1), "a");
    }

    #[test]
    fn test0z() {
        assert_eq!(&Solution::get_smallest_string(1, 26), "z");
    }

    #[test]
    fn test0az() {
        assert_eq!(&Solution::get_smallest_string(2, 27), "az");
    }

    #[test]
    fn example1() {
        assert_eq!(&Solution::get_smallest_string(3, 27), "aay");
    }

    #[test]
    fn example2() {
        assert_eq!(&Solution::get_smallest_string(5, 73), "aaszz");
    }
}
