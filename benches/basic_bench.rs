#![feature(test)]

extern crate intmap;
extern crate rand;
extern crate test;
extern crate fnv;

use intmap::IntMap;
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use fnv::{FnvHashMap, FnvHashSet};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::hash::{Hasher, BuildHasherDefault};

    // "hash" a u64 to itself
    pub struct IdentityHasher(u64);

    impl Default for IdentityHasher {
        #[inline]
        fn default() -> IdentityHasher {
            IdentityHasher(0)
        }
    }

    impl Hasher for IdentityHasher {
        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            unimplemented!();
        }

        #[inline]
        fn write_u64(&mut self, i: u64) {
            self.0 = i;
        }
    }

    /// A builder for default Identity hashers. (cribbed from FNV)
    pub type IdentityBuildHasher = BuildHasherDefault<IdentityHasher>;

    /// A `HashMap` using a default Const hasher.
    pub type IdentityHashMap<K, V> = HashMap<K, V, IdentityBuildHasher>;

    /// A `HashSet` using a default Const hasher.
    pub type IdentityHashSet<T> = HashSet<T, IdentityBuildHasher>;

    const VEC_COUNT: usize = 2_000;

    #[bench]
    fn u64_insert_sip_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

        });
    }

    #[bench]
    fn u64_insert_btree_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = BTreeMap::new();

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

        });
    }

    #[bench]
    fn u64_insert_fnv_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = FnvHashMap::with_capacity_and_hasher(data.len(), Default::default());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_identity_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = IdentityHashMap::with_capacity_and_hasher(data.len(), Default::default());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

        });
    }

    #[bench]
    fn u64_insert_sip_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = HashSet::with_capacity(data.len());

        b.iter(|| {
            set.clear();
            for s in data.iter() {
                test::black_box(set.insert(s));
            }

        });
    }

    #[bench]
    fn u64_insert_fnv_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = FnvHashSet::with_capacity_and_hasher(data.len(), Default::default());

        b.iter(|| {
            set.clear();
            for s in data.iter() {
                test::black_box(set.insert(s));
            }
        });
    }

    #[bench]
    fn u64_insert_identity_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = IdentityHashSet::with_capacity_and_hasher(data.len(), Default::default());

        b.iter(|| {
            set.clear();
            for s in data.iter() {
                test::black_box(set.insert(s));
            }

        });
    }

    #[bench]
    fn u64_insert_btree_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = BTreeSet::new();

        b.iter(|| {
            set.clear();
            for s in data.iter() {
                test::black_box(set.insert(s));
            }

        });
    }

    #[bench]
    fn u64_get_sip_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = HashSet::with_capacity(data.len());
        for s in data.iter() {
            set.insert(s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    set.contains(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_fnv_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = FnvHashSet::with_capacity_and_hasher(data.len(), Default::default());
        for s in data.iter() {
            set.insert(s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    set.contains(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_identity_hash_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut set = IdentityHashSet::with_capacity_and_hasher(data.len(), Default::default());
        for s in data.iter() {
            set.insert(s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    set.contains(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_btree_set(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = BTreeSet::new();

        for s in data.iter() {
            map.insert(s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_identity_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = IdentityHashMap::with_capacity_and_hasher(data.len(), Default::default());
        for s in data.iter() {
            map.insert(s, s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_fnv_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = FnvHashMap::with_capacity_and_hasher(data.len(), Default::default());
        for s in data.iter() {
            map.insert(s, s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_sip_hash_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());
        for s in data.iter() {
            map.insert(s, s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s)
                });
            }
        });
    }

    #[bench]
    fn u64_get_btree_map(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = BTreeMap::new();
        for s in data.iter() {
            map.insert(s, s);
        }
        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s)
                });
            }
        });
    }

    #[bench]
    fn u64_insert_intmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, s));
            }
        });
    }

    #[bench]
    fn u64_get_intmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        let mut map = IntMap::with_capacity(data.len());
        for s in data.iter() {
            map.insert(*s, s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(*s)
                });
            }
        });
    }


    fn get_random_range(count: usize) -> Vec<u64> {
        use rand::{Rng, SeedableRng, StdRng};

        let mut vec = Vec::new();

        let seed: &[_] = &[4, 2, 4, 2];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for _ in 0..count {
            vec.push(rng.gen::<u64>());
        }

        vec.sort();
        vec.dedup();

        vec
    }
}
