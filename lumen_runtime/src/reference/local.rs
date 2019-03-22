use std::sync::atomic::{AtomicU64, Ordering};

use crate::term::{Tag::LocalReference, Term};

pub struct Reference {
    #[allow(dead_code)]
    header: Term,
    pub number: u64,
}

impl Reference {
    pub fn new(number: u64) -> Reference {
        Reference {
            header: Term {
                tagged: LocalReference as usize,
            },
            number,
        }
    }

    pub fn next() -> Reference {
        Self::new(COUNT.fetch_add(1, Ordering::SeqCst))
    }
}

// References are always 64-bits even on 32-bit platforms
static COUNT: AtomicU64 = AtomicU64::new(0);
