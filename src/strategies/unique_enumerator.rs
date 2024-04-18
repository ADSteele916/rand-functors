use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::marker::PhantomData;

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Inner, RandomStrategy, RandomVariable, RandomVariableRange};

#[cfg(feature = "std")]
/// Produces all possible outputs of the random process, without repetition,
/// stored in a [`HashSet`].
///
/// `UniqueEnumerator` is optimal in scenarios where certain operations will map
/// many inputs to the same output and the user does not care about the relative
/// frequencies of possible outputs.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UniqueEnumerator<S: BuildHasher + Default = RandomState> {
    phantom: PhantomData<S>,
}

#[cfg(feature = "std")]
impl<S: BuildHasher + Default> RandomStrategy for UniqueEnumerator<S> {
    type Functor<I: Inner> = HashSet<I, S>;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        f.into_iter().map(func).collect()
    }

    #[inline]
    fn fmap_flat<A: Inner, B: Inner, F: FnMut(A) -> Self::Functor<B>>(
        f: Self::Functor<A>,
        _: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B> {
        f.into_iter().flat_map(func).collect()
    }

    #[inline]
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

    #[inline]
    fn fmap_rand_range<A: Inner, B: Inner, R: RandomVariable + SampleUniform, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        range: impl RandomVariableRange<R>,
        _: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>,
    {
        f.into_iter()
            .flat_map(|a| range.sample_space().map(move |r| (a.clone(), r)))
            .map(|(a, r)| func(a, r))
            .collect()
    }
}
