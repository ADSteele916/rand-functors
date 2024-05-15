use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{
    FlattenableRandomStrategy, Inner, RandomStrategy, RandomVariable, RandomVariableRange,
};

/// Samples the desired distributions and produces a single possible output of
/// the random process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Sampler;

impl RandomStrategy for Sampler {
    type Functor<I: Inner> = I;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        func(f)
    }

    #[inline]
    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: FnOnce(A, R) -> B>(
        f: Self::Functor<A>,
        rng: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        func(f, rng.gen())
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
        func(f, rng.gen_range(range))
    }
}

impl FlattenableRandomStrategy for Sampler {
    #[inline]
    fn fmap_flat<A: Inner, B: Inner, F: FnMut(A) -> Self::Functor<B>>(
        f: Self::Functor<A>,
        mut func: F,
    ) -> Self::Functor<B> {
        func(f)
    }
}
