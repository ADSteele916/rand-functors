use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::{Functor, Inner};

impl<I: Inner> Functor<I> for I {
    type Output<O: Inner> = O;

    fn pure(i: I) -> I {
        i
    }

    fn fmap<O: Inner, F: Fn(I) -> O>(self, func: F) -> Self::Output<O> {
        func(self)
    }
}

impl<I: Inner> Functor<I> for Vec<I> {
    type Output<O: Inner> = Vec<O>;

    fn pure(i: I) -> Self {
        vec![i]
    }

    fn fmap<O: Inner, F: Fn(I) -> O>(self, func: F) -> Self::Output<O> {
        self.into_iter().map(func).collect()
    }
}

impl<I: Inner, S: BuildHasher + Default> Functor<I> for HashMap<I, usize, S> {
    type Output<O: Inner> = HashMap<O, usize, S>;

    fn pure(i: I) -> Self {
        let mut hm = Self::default();
        hm.insert(i, 1);
        hm
    }

    fn fmap<O: Inner, F: Fn(I) -> O>(self, func: F) -> Self::Output<O> {
        // Constructing a new HashMap is necessary, as there may be fewer new
        // keys than old keys, which requires merging some or all counts.
        let mut out = Self::Output::<O>::with_capacity_and_hasher(self.len(), Default::default());
        self.into_iter()
            .map(|(i, count)| (func(i), count))
            .for_each(|(o, count)| {
                *out.entry(o).or_insert(0) += count;
            });
        out
    }
}
