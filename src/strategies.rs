use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, RandomState};
use std::marker::PhantomData;

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{Inner, RandomStrategy, RandomVariable, RandomVariableRange};

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

#[inline(always)]
fn vec_fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
    f: Vec<A>,
    func: F,
) -> Vec<B>
where
    Standard: Distribution<R>,
{
    f.into_iter()
        .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
        .map(|(a, r)| func(a, r))
        .collect()
}

#[inline(always)]
fn vec_fmap_rand_range<A: Inner, B: Inner, R: RandomVariable + SampleUniform, F: Fn(A, R) -> B>(
    f: Vec<A>,
    range: impl RandomVariableRange<R>,
    func: F,
) -> Vec<B>
where
    Standard: Distribution<R>,
{
    f.into_iter()
        .flat_map(|a| range.sample_space().map(move |r| (a.clone(), r)))
        .map(|(a, r)| func(a, r))
        .collect()
}

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
        Self::shrink_to_capacity(vec_fmap_rand(f, func), rng)
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
        Self::shrink_to_capacity(vec_fmap_rand_range(f, range, func), rng)
    }
}

/// Produces all possible outputs of the random process, with repetition, as a
/// [`Vec`].
///
/// `Enumerator` can be preferable to [`Counter`] in applications where the
/// functions passed to `fmap_rand` do not typically produce the same value for
/// different random inputs. In these cases, using [`Counter`], which is backed
/// by a [`HashMap`] functor, will often not result in the expected space
/// savings, as hash tables will over-allocate to maintain an acceptable load
/// factor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Enumerator;

impl RandomStrategy for Enumerator {
    type Functor<I: Inner> = Vec<I>;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        f.into_iter().map(func).collect()
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
        vec_fmap_rand(f, func)
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
        vec_fmap_rand_range(f, range, func)
    }
}

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

impl<S: BuildHasher + Default> RandomStrategy for UniqueEnumerator<S> {
    type Functor<I: Inner> = HashSet<I, S>;

    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        f.into_iter().map(func).collect()
    }

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

/// Produces all possible outputs of the random process, with repetition, stored
/// in a [`HashMap`].
///
/// `Counter` is optimal in scenarios where certain operations will map many
/// inputs to the same output. Examples include conditionally zeroing out a
/// field of a struct or the use of functions like `saturating_add` or
/// `saturating_mul`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Counter<S: BuildHasher + Default = RandomState> {
    phantom: PhantomData<S>,
}

impl<S: BuildHasher + Default> RandomStrategy for Counter<S> {
    type Functor<I: Inner> = HashMap<I, usize, S>;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        // Constructing a new HashMap is necessary, as there may be fewer new
        // keys than old keys, which requires merging some or all counts.
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .map(|(i, count)| (func(i), count))
            .for_each(|(o, count)| {
                *new_functor.entry(o).or_insert(0) += count;
            });
        new_functor
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
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
            .map(|((a, c), r)| (func(a, r), c))
            .for_each(|(b, count)| {
                *new_functor.entry(b).or_insert(0) += count;
            });
        new_functor
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
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .flat_map(|a| range.sample_space().map(move |r| (a.clone(), r)))
            .map(|((a, c), r)| (func(a, r), c))
            .for_each(|(b, count)| {
                *new_functor.entry(b).or_insert(0) += count;
            });
        new_functor
    }
}
