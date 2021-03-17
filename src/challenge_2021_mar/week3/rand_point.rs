// Generate Random Point in a Circle
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3675/

use std::time::SystemTime;

pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    state: u64,
}

impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            state: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn random(&mut self) -> f64 {
        // A linear congruential generator (assuming third-party modules are
        // not available in Leetcode)
        self.state = self.state.wrapping_mul(134775813).wrapping_add(1);
        (self.state >> (64 - 53)) as f64 * 2_f64.powi(-53)
    }

    pub fn rand_point(&mut self) -> Vec<f64> {
        // There is no TAU in Leetcode's version of rust
        let th = self.random() * 2.0 * std::f64::consts::PI;
        // Circumference is proportinal to radius.  Probability density
        // function is `2*r` (coef 2 is to get total 1 when integrated).
        // Cumulative distribution function (integrated PDF) is `r*r`.
        // Thus `r=sqrt(CDF)`.
        let r = self.radius * self.random().sqrt();
        let x = r * th.cos() + self.x_center;
        let y = r * th.sin() + self.y_center;
        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_uniform() {
        let mut sol = Solution::new(1.0, 0.0, 0.0);
        let mut counts = vec![0; 10];
        for _ in 0..10_000 {
            let bucket = (sol.random() * 10.0) as usize;
            counts[bucket] += 1;
        }
        assert!(counts.iter().all(|&count| 900 < count && count < 1_100));
    }

    #[test]
    fn test_rand_point_radius_uniform() {
        let mut counts = vec![0; 10];
        let mut sol = Solution::new(10.0, 3.0, -10.0);
        for _ in 0..100_000 {
            let res = sol.rand_point();
            let dist =
                ((res[0] - 3.0).powi(2) + (res[1] + 10.0).powi(2)).sqrt();
            counts[dist as usize] += 1;
        }
        assert!(counts.iter().enumerate().all(|(idx, &count)| {
            let base = idx * 2 + 1;
            base * 900 < count && count < base * 1_100
        }));
    }
}
