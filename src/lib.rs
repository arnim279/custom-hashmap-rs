#![feature(test)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<'a, K: Hash + PartialEq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::new_with_custom_range(100)
    }

    pub fn new_with_custom_range(range_count: usize) -> Self {
        let mut buckets = Vec::new();
        while buckets.len() < range_count {
            buckets.push(Vec::new());
        }

        Self { buckets }
    }

    pub fn set(&mut self, key: K, value: V) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        let index = hash % self.buckets.len();

        let range = self.buckets.get_mut(index).unwrap();

        for item in range.iter() {
            if item.0 == key {
                return;
            }
        }

        range.push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        let index = hash % self.buckets.len();

        let range = self.buckets.get(index).unwrap();

        for item in range.iter() {
            if &item.0 == key {
                return Some(&item.1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn test_hashmap_int() {
        use super::HashMap;

        let mut map = HashMap::new();
        map.set(1, 2);
        assert_eq!(map.get(&1), Some(&2))
    }

    #[test]
    fn test_hashmap_str() {
        use super::HashMap;

        let mut map = HashMap::new();
        map.set("hello", "world");
        assert_eq!(map.get(&"hello"), Some(&"world"))
    }

    use std::ops::Range;
    use test::{black_box, Bencher};

    fn get_data() -> Range<isize> {
        0..50_000
    }

    #[bench]
    fn bench_std_hashmap(bencher: &mut Bencher) {
        use std::collections::hash_map::HashMap;

        bencher.iter(|| {
            let mut map = HashMap::new();
            let fill_iterator = get_data();
            for v in fill_iterator {
                black_box(map.insert(v, v));
            }

            let get_iterator = get_data();
            for v in get_iterator {
                black_box(map.get(&v));
            }
        })
    }

    #[bench]
    fn bench_custom_hashmap(bencher: &mut Bencher) {
        use super::HashMap;

        bencher.iter(|| {
            let mut map: HashMap<isize, isize> = HashMap::new_with_custom_range(1_000);
            let fill_iterator = get_data();
            for v in fill_iterator {
                black_box(map.set(v, v));
            }

            let get_iterator = get_data();
            for v in get_iterator {
                black_box(map.get(&v));
            }
        })
    }
}
