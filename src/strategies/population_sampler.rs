use alloc::vec::Vec;

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Inner, RandomStrategy, RandomVariable, RandomVariableRange};

/// Produces a random subset (technically, submultiset) of possible outputs of
/// the random process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct PopulationSampler<const N: usize>;

impl<const N: usize> PopulationSampler<N> {
    #[inline(always)]
    fn shrink_to_capacity<T: Inner>(mut f: Vec<T>, rng: &mut impl Rng) -> Vec<T> {
        while f.len() > N {
            let index = rng.gen_range(0..f.len());
            f.swap_remove(index);
        }
        f
    }
}

impl<const N: usize> RandomStrategy for PopulationSampler<N> {
    type Functor<I: Inner> = Vec<I>;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        f.into_iter().map(func).collect()
    }

    #[inline]
    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        rng: &mut impl Rng,
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
            rng,
        )
    }

    #[inline]
    fn fmap_rand_range<A: Inner, B: Inner, R: RandomVariable + SampleUniform, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        range: impl RandomVariableRange<R>,
        rng: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        Self::shrink_to_capacity(
            f.into_iter()
                .flat_map(|a| range.sample_space().map(move |r| (a.clone(), r)))
                .map(|(a, r)| func(a, r))
                .collect(),
            rng,
        )
    }
}
