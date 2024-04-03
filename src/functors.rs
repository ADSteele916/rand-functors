use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

use crate::{Functor, Inner};

impl<I: Inner> Functor<I> for I {
    #[inline]
    fn pure(i: I) -> I {
        i
    }
}

impl<I: Inner> Functor<I> for Vec<I> {
    #[inline]
    fn pure(i: I) -> Self {
        vec![i]
    }
}

impl<I: Inner, S: BuildHasher + Default> Functor<I> for HashMap<I, usize, S> {
    #[inline]
    fn pure(i: I) -> Self {
        let mut hm = Self::default();
        hm.insert(i, 1);
        hm
    }
}

impl<I: Inner, S: BuildHasher + Default> Functor<I> for HashSet<I, S> {
    fn pure(i: I) -> Self {
        let mut hs = Self::default();
        hs.insert(i);
        hs
    }
}
