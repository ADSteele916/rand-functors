use std::collections::HashMap;
use std::hash::{BuildHasher, RandomState};
use std::marker::PhantomData;

use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Inner, RandomStrategy, RandomVariable};

/// Samples the desired distributions and produces a single possible output of
/// the random process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Sampler;

impl RandomStrategy for Sampler {
    type Functor<I: Inner> = I;

    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: FnOnce(A, R) -> B>(
        f: Self::Functor<A>,
        rand: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        func(f, rand.gen())
    }
}

/// Produces a random subset (technically, submultiset) of possible outputs of
/// the random process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct PopulationSampler<const N: usize>;

impl<const N: usize> PopulationSampler<N> {
    fn shrink_to_capacity<T: Inner>(
        mut f: <Self as RandomStrategy>::Functor<T>,
        rand: &mut impl Rng,
    ) -> <Self as RandomStrategy>::Functor<T> {
        while f.len() > N {
            let index = rand.gen_range(0..f.len());
            f.swap_remove(index);
        }
        f
    }
}

impl<const N: usize> RandomStrategy for PopulationSampler<N> {
    type Functor<I: Inner> = Vec<I>;

    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        rand: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        Self::shrink_to_capacity(
            f.into_iter()
                .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
                .map(|(a, r)| func(a, r))
                .collect(),
            rand,
        )
    }
}

/// Produces all possible outputs of the random process, with repetition, as a
/// [`Vec`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Enumerator;

impl RandomStrategy for Enumerator {
    type Functor<I: Inner> = Vec<I>;

    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        _: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        f.into_iter()
            .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
            .map(|(a, r)| func(a, r))
            .collect()
    }
}

/// Produces all possible outputs of the random process, with repetition, stored
/// in a [`HashMap`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Counter<S: BuildHasher + Default = RandomState> {
    phantom: PhantomData<S>,
}

impl<S: BuildHasher + Default> RandomStrategy for Counter<S> {
    type Functor<I: Inner> = HashMap<I, usize, S>;

    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        _: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        let mut out = Self::Functor::<B>::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
            .map(|((a, c), r)| (func(a, r), c))
            .for_each(|(b, count)| {
                *out.entry(b).or_insert(0) += count;
            });
        out
    }
}
