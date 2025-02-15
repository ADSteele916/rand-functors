#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::hash::BuildHasher;

#[cfg(feature = "std")]
use num::One;

use crate::{Functor, Inner};

impl<I: Inner> Functor<I> for I {
    #[inline]
    fn pure(i: I) -> I {
        i
    }
}

#[cfg(feature = "alloc")]
impl<I: Inner> Functor<I> for Vec<I> {
    #[inline]
    fn pure(i: I) -> Self {
        vec![i]
    }
}

#[cfg(feature = "std")]
impl<I: Inner, N: Clone + Default + One, S: BuildHasher + Default> Functor<I> for HashMap<I, N, S> {
    #[inline]
    fn pure(i: I) -> Self {
        let mut hm = Self::default();
        hm.insert(i, N::one());
        hm
    }
}

#[cfg(feature = "std")]
impl<I: Inner, S: BuildHasher + Default> Functor<I> for HashSet<I, S> {
    fn pure(i: I) -> Self {
        let mut hs = Self::default();
        hs.insert(i);
        hs
    }
}
