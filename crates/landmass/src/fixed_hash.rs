//! Taking Tehran's patch: `landmass`'s hash maps and sets with a fixed hasher.
//!
//! `std`'s are seeded at random for each map, and `landmass` walks some of them (a node's
//! off-mesh links, the islands to link) in their order, which then differs from one archipelago
//! to the next: two runs over the same islands could find different paths, and the game wouldn't
//! play out the same way twice. With a fixed hasher, the same inputs walk them in the same order.

use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

pub(crate) type HashMap<K, V> =
  std::collections::HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
pub(crate) type HashSet<K> =
  std::collections::HashSet<K, BuildHasherDefault<DefaultHasher>>;
