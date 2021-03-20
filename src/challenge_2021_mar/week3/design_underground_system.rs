// Design Underground System
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3678/

use std::collections::HashMap;

#[derive(Default)]
pub struct UndergroundSystem {
    // id => (station_name, t)
    checked_in: HashMap<i32, (String, i32)>,
    // (start_station, end_station) => (t_total, count)
    stats: HashMap<(String, String), (f64, u32)>,
}

impl UndergroundSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checked_in.insert(id, (station_name, t));
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, t_from) = self.checked_in.remove(&id).unwrap();
        let &mut (ref mut t_total, ref mut count) =
            self.stats.entry((start_station, station_name)).or_default();
        *t_total += (t - t_from) as f64;
        *count += 1;
    }

    pub fn get_average_time(
        &self,
        start_station: String,
        end_station: String,
    ) -> f64 {
        let &(t_total, count) =
            self.stats.get(&(start_station, end_station)).unwrap();
        t_total / count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn example1() {
        let mut us = UndergroundSystem::new();
        us.check_in(45, "Leyton".into(), 3);
        us.check_in(32, "Paradise".into(), 8);
        us.check_in(27, "Leyton".into(), 10);
        us.check_out(45, "Waterloo".into(), 15);
        us.check_out(27, "Waterloo".into(), 20);
        us.check_out(32, "Cambridge".into(), 22);
        assert_approx_eq!(
            us.get_average_time("Paradise".into(), "Cambridge".into()),
            14.0
        );
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0
        );
        us.check_in(10, "Leyton".into(), 24);
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0
        );
        us.check_out(10, "Waterloo".into(), 38);
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Waterloo".into()),
            12.0
        );
    }

    #[test]
    fn example2() {
        let mut us = UndergroundSystem::new();
        us.check_in(10, "Leyton".into(), 3);
        us.check_out(10, "Paradise".into(), 8);
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Paradise".into()),
            5.0
        );
        us.check_in(5, "Leyton".into(), 10);
        us.check_out(5, "Paradise".into(), 16);
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Paradise".into()),
            5.5
        );
        us.check_in(2, "Leyton".into(), 21);
        us.check_out(2, "Paradise".into(), 30);
        assert_approx_eq!(
            us.get_average_time("Leyton".into(), "Paradise".into()),
            6.666667
        );
    }
}
