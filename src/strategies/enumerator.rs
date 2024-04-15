use alloc::vec::Vec;

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Inner, RandomStrategy, RandomVariable, RandomVariableRange};

#[cfg(feature = "alloc")]
/// Produces all possible outputs of the random process, with repetition, as a
/// [`Vec`].
///
/// `Enumerator` can be preferable to [`Counter`] in applications where the
/// functions passed to `fmap_rand` do not typically produce the same value for
/// different random inputs. In these cases, using [`Counter`], which is backed
/// by a [`HashMap`] functor, will often not result in the expected space
/// savings, as hash tables will over-allocate to maintain an acceptable load
/// factor.
///
/// [`Counter`]: crate::Counter
/// [`HashMap`]: std::collections::HashMap
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Enumerator;

#[cfg(feature = "alloc")]
impl RandomStrategy for Enumerator {
    type Functor<I: Inner> = Vec<I>;

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
