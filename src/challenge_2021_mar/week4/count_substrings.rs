// Palindromic Substrings
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3686/

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        // Using `s.chars().collect()` is more correct, but Leetcode uses ASCII
        // so we can save one allocation.
        let s = s.as_bytes();

        let mut radii = vec![0; 2 * s.len() - 1];
        let mut center = 0;
        let mut radius = 0;
        while center < radii.len() {
            if (center - radius) % 2 != 0 {
                radius += 1;
            }
            while center >= radius
                && center + radius < radii.len()
                && s[(center - radius + 1) / 2] == s[(center + radius + 1) / 2]
            {
                radius += 2;
            }
            radii[center] = radius;
            let mut inner_radius = 1;
            while center >= inner_radius
                && center + inner_radius < radii.len()
                && inner_radius + radii[center - inner_radius] < radius
            {
                radii[center + inner_radius] = radii[center - inner_radius];
                inner_radius += 1;
            }
            center += inner_radius;
            radius -= inner_radius;
        }

        radii.iter().fold(0, |count, radius| count + radius / 2) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_substrings("abc".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_substrings("aaa".into()), 6);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::count_substrings(
                "bbccaacacdbdbcbcbbbcbadcbdddbabaddbcadb".into()
            ),
            64
        );
    }
}
