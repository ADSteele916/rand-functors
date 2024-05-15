use alloc::vec::Vec;

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Enumerator, Inner, RandomStrategy, RandomVariable, RandomVariableRange};

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
        Enumerator::fmap(f, func)
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
        Self::shrink_to_capacity(Enumerator::fmap_rand(f, rng, func), rng)
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
        Self::shrink_to_capacity(Enumerator::fmap_rand_range(f, range, rng, func), rng)
    }
}
