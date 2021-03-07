pub struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self {
            buckets: vec![vec![]; 1024],
        }
    }

    fn index(&self, key: i32) -> usize {
        // It's too dump to be called hash, but who cares?
        let hash = key
            ^ key.rotate_left(8)
            ^ key.rotate_left(16)
            ^ key.rotate_left(24);
        hash as usize % self.buckets.len()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let idx = self.index(key);
        let bucket = &mut self.buckets[idx];
        for (k, v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        bucket.push((key, value));
    }

    pub fn get(&self, key: i32) -> i32 {
        let idx = self.index(key);
        let bucket = &self.buckets[idx];
        for (k, v) in bucket.iter() {
            if *k == key {
                return *v;
            }
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let idx = self.index(key);
        let bucket = &mut self.buckets[idx];
        bucket.retain(|(k, _)| *k != key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut hm = MyHashMap::new();
        hm.put(1, 1);
        hm.put(2, 2);
        assert_eq!(hm.get(1), 1);
        assert_eq!(hm.get(3), -1);
        hm.put(2, 1);
        assert_eq!(hm.get(2), 1);
        hm.remove(2);
        assert_eq!(hm.get(2), -1);
    }
}
