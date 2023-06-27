use crate::Storage;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

impl PartialOrd for Storage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Storage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl Eq for Storage {}

impl Hash for Storage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Default for Storage {
    /// Constructs an empty string.
    #[inline]
    fn default() -> Self {
        Storage {
            bytes: Default::default(),
        }
    }
}

impl Deref for Storage {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}
