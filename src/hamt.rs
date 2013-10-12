//! An unordered map and set type implemented as hash array mapped tries
//!
//! The tries use a keyed hash with new random keys generated for each container, so the ordering
//! of a set of keys in a hash table is randomized.
//!
//! Unlike hash tables, hash array mapped tries are persistent.

use std::container::{Container, Map, Set};
use std::cmp::{Eq, Equiv};
use std::hash::Hash;
use std::option::{None, Option, Some};
use std::{u8, uint};

#[cfg(target_word_size = "64")]
static LOG_UINT_SIZE: u64 = 6;

#[cfg(target_word_size = "32")]
static LOG_UINT_SIZE: u64 = 5;

#[cfg(target_word_size = "16")]
static LOG_UINT_SIZE: u64 = 4;

#[cfg(target_word_size = "8")]
static LOG_UINT_SIZE: u64 = 3;

/// returns index to bitset and remaining hash for next round
fn split_hash(h: u64) -> (uint, u64) {
    (h as uint & ((uint::max_value - 1) >> 1), h >>  LOG_UINT_SIZE)
}

struct Bucket<K,V> {
    key: K,
    value: V
}

// Arrays are deceptively fixed size to avoid boxing. Always allocate memory and then cast to HAMT<T> !

enum HAMT<K,V> {
    Buckets (u64, [Bucket<K,V>, ..uint::max_value] ),
    Branches (uint, [@HAMT<K,V>, ..uint::bits] )
}

impl<K: Hash + Eq, V: Clone> HAMT<K,V> {
    fn find(@self, key: K, hash: u64)
            -> Option<V> {

        let mut current: @HAMT<K,V> = self;
        let mut partial: u64 = hash;

        loop {
            match *current {
                Buckets (h, ref buckets) => return match h == hash {
                    true => buckets.iter().find(|b| key.eq(&b.key)).map(|b| b.value.clone()),
                    false => None
                },
                Branches (bitset, ref branches) => match bitset & (1u << partial) {
                    0 => return None,
                    _ => {
                        let (index, p2) = split_hash(partial);
                        partial = p2;
                        current = branches[index];
                    }
                }
            }
        }
    }
}



#[allow(missing_doc)]
pub struct HashMap<K,V> {
    priv size: uint,
    priv map: HAMT<K,V>
}

impl<K:Hash + Eq,V> Container for HashMap<K, V> {
    /// Return the number of elements in the map
    fn len(&self) -> uint { self.size }
}

impl<K:Hash + Eq,V> Map<K, V> for HashMap<K, V> {
    /// Return a reference to the value corresponding to the key
    fn find<'a>(&'a self, k: &K) -> Option<&'a V> {
        match self.bucket_for_key(k) {
            FoundEntry(idx) => Some(self.value_for_bucket(idx)),
            TableFull | FoundHole(_) => None,
        }
    }
}



#[allow(missing_doc)]
pub struct HashSet<T> {
    priv map: HashMap<T, ()>
}

impl<T:Hash + Eq> Eq for HashSet<T> {
    fn eq(&self, other: &HashSet<T>) -> bool { self.map == other.map }
    fn ne(&self, other: &HashSet<T>) -> bool { self.map != other.map }
}

impl<T:Hash + Eq> Container for HashSet<T> {
    /// Return the number of elements in the set
    fn len(&self) -> uint { self.map.len() }
}

impl<T:Hash + Eq> Set<T> for HashSet<T> {
    /// Return true if the set contains a value
    fn contains(&self, value: &T) -> bool { self.map.contains_key(value) }

    /// Return true if the set has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        self.iter().all(|v| !other.contains(v))
    }

    /// Return true if the set is a subset of another
    fn is_subset(&self, other: &HashSet<T>) -> bool {
        self.iter().all(|v| other.contains(v))
    }

    /// Return true if the set is a superset of another
    fn is_superset(&self, other: &HashSet<T>) -> bool {
        other.is_subset(self)
    }
}
