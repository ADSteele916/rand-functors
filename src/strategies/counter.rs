use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::marker::PhantomData;

use num::traits::{NumAssign, Unsigned};
use num::Integer;
use rand::distr::uniform::SampleUniform;
use rand::distr::StandardUniform;
use rand::prelude::*;

use crate::{
    FlattenableRandomStrategy, Inner, RandomStrategy, RandomVariable, RandomVariableRange,
};

/// Produces all possible outputs of the random process, with repetition, stored
/// in a [`HashMap`].
///
/// `Counter` is optimal in scenarios where certain operations will map many
/// inputs to the same output. Examples include conditionally zeroing out a
/// field of a struct or the use of functions like `saturating_add` or
/// `saturating_mul`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Counter<
    S: BuildHasher + Default = RandomState,
    N: Clone + Default + NumAssign + Unsigned = usize,
> {
    count_phantom: PhantomData<N>,
    hasher_phantom: PhantomData<S>,
}

impl<S: BuildHasher + Default, N: Clone + Default + NumAssign + Unsigned> RandomStrategy
    for Counter<S, N>
{
    type Functor<I: Inner> = HashMap<I, N, S>;

    #[inline]
    fn fmap<A: Inner, B: Inner, F: Fn(A) -> B>(f: Self::Functor<A>, func: F) -> Self::Functor<B> {
        // Constructing a new HashMap is necessary, as there may be fewer new
        // keys than old keys, which requires merging some or all counts.
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .map(|(i, count)| (func(i), count))
            .for_each(|(o, count)| {
                *new_functor.entry(o).or_insert(N::zero()) += count;
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
        StandardUniform: Distribution<R>,
    {
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .flat_map(|a| R::sample_space().map(move |r| (a.clone(), r)))
            .map(|((a, c), r)| (func(a, r), c))
            .for_each(|(b, count)| {
                *new_functor.entry(b).or_insert(N::zero()) += count;
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
        StandardUniform: Distribution<R>,
    {
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        f.into_iter()
            .flat_map(|a| range.sample_space().map(move |r| (a.clone(), r)))
            .map(|((a, c), r)| (func(a, r), c))
            .for_each(|(b, count)| {
                *new_functor.entry(b).or_insert(N::zero()) += count;
            });
        new_functor
    }
}

impl<S: BuildHasher + Default, N: Clone + Default + Integer + NumAssign + Unsigned>
    FlattenableRandomStrategy for Counter<S, N>
{
    #[inline]
    fn fmap_flat<A: Inner, B: Inner, F: FnMut(A) -> Self::Functor<B>>(
        f: Self::Functor<A>,
        mut func: F,
    ) -> Self::Functor<B> {
        let mut new_functor = Self::Functor::with_capacity_and_hasher(f.len(), Default::default());
        let children = f
            .into_iter()
            .map(|(i, outer_count)| {
                let functor = func(i);
                let inner_count_sum = functor
                    .values()
                    .fold(N::zero(), |sum, inner_count| sum + inner_count.clone());
                (functor, outer_count, inner_count_sum)
            })
            .collect::<Vec<_>>();
        let Some(inner_count_lcm) =
            children
                .iter()
                .fold(None, |lcm: Option<N>, (_, _, inner_count_sum)| {
                    if let Some(lcm) = lcm {
                        Some(num::integer::lcm(lcm, inner_count_sum.clone()))
                    } else {
                        Some(inner_count_sum.clone())
                    }
                })
        else {
            return new_functor;
        };
        for (child, outer_count, inner_count_sum) in children {
            let scaling = inner_count_lcm.clone() / inner_count_sum;
            for (output, inner_count) in child {
                *new_functor.entry(output).or_insert(N::zero()) +=
                    scaling.clone() * inner_count * outer_count.clone();
            }
        }
        new_functor
    }
}
