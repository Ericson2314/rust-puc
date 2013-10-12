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
use std::uint;

// ~ because one should always alias the node owning the array, not the array itself.

enum HAMT<K,V> {
    Buckets (u64, ~[(K,V)] ),
    Node (uint, ~[@HAMT<K,V>] )
}



#[allow(missing_doc)]
pub struct HashMap<K,V> {
    priv size: uint,
    priv map: HAMT<K,V>
}



#[allow(missing_doc)]
pub struct HashSet<T> {
    priv map: HashMap<T, ()>
}
