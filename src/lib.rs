//! `rand-functors` provides an abstraction over different ways of evaluating
//! random processes expressed as functions of both deterministic and stochastic
//! data. This is achieved using a combination of a type-based version of the
//! Strategy pattern and functional programming's Functor pattern.
//!
//! A motivating problem for this crate is the code duplication present across
//! these two functions modelling the same random process:
//! ```
//! use rand::prelude::*;
//!
//! fn next_state(mut state: u8) -> u8 {
//!     state = state.wrapping_add(random());
//!     if random() {
//!         state %= 3;
//!     }
//!     state
//! }
//!
//! fn next_states(state: u8) -> Vec<u8> {
//!     let mut out: Vec<_> = (0..=255).map(|r| state.wrapping_add(r)).collect();
//!     out.append(&mut out.iter().copied().map(|i| i % 3).collect());
//!     out
//! }
//! ```
//! While these functions may appear different, the same random process is
//! embedded in both of them. A random `u8` is added to `state` and then, if a
//! random `bool` is `true`, the state will be set to itself modulo 3.
//!
//! This redundant implementation of the random process could pose issues during
//! a refactor. If one decides to change the `%= 3` to a `%= 5` in `next_state`,
//! he or she will need to make the corresponding update in `next_states`.
//!
//! Using `rand-functors`, these two functions can be combined as:
//! ```
//! use rand::prelude::*;
//! use rand_functors::{Functor, RandomStrategy};
//!
//! fn next_state<S: RandomStrategy>(state: u8) -> S::Functor<u8> {
//!     let mut out = S::fmap_rand(Functor::pure(state), &mut thread_rng(), |s, r| {
//!         s.wrapping_add(r)
//!     });
//!     out = S::fmap_rand(out, &mut thread_rng(), |s, r| if r { s % 3 } else { s });
//!     out
//! }
//! ```
//! This new implementation makes `next_state` generic over a [`RandomStrategy`]
//! `S`. Its return type is also changed to the [`Functor`] associated with `S`.
//! Inside, `state` is converted from `u8` to `S::Functor<u8>`. The remainder of
//! the function is essentially the same as the original `next_state`, but each
//! operation a random sample is now wrapped in a call to `S::fmap_rand`.
//! Calling `next_state::<Sampler>(s)` would be equivalent to calling
//! `next_state(s)` before. Similarly, one could call
//! `next_state::<Enumerator>(s)` instead of using `next_states(s)`, which would
//! require maintaining a separate implementation of the same core process.
//!
//! At present, `rand-functors` only supports random variables that are either
//! of type [`bool`] or of a numeric type occupying no more than 16 bits by
//! default. However, it is possible to implement all the requisite traits for a
//! custom data type.

#![warn(clippy::cargo)]
#![warn(missing_docs)]

pub use strategies::*;

mod functors;
mod random_variables;
mod strategies;

use std::hash::Hash;

use rand::distributions::Standard;
use rand::prelude::*;

/// A strategy for evaluating sequences of functions of random data.
///
/// Types implementing `RandomStrategy` are typically not constructed. For this
/// same reason, they are typically unit structs. Behaviour should be specified
/// at compile-time, to allow calls to `fmap_rand` and `Functor::fmap` to be
/// properly inlined.
pub trait RandomStrategy {
    /// The functor that this strategy operates on.
    ///
    /// Functions using a given strategy will typically return its associated
    /// functor in the form `S::Functor<T>`.
    type Functor<I: Inner>: Functor<I>;

    /// Using the strategy specified by the implementor, apply the given binary
    /// function to the given functor and an element of the sample space of a
    /// [`RandomVariable`].
    ///
    /// Note that **no guarantees** are made about whether or how the `rand`
    /// parameter will be used. It may be sampled zero, one, or arbitrarily many
    /// times. It may be used to sample values of type `R`, of type [`usize`],
    /// or some other type. If some model of the random number generator is
    /// available, then that model should be responsible for enumerating
    /// possible outcomes.
    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        rng: &mut impl Rng,
        func: F,
    ) -> Self::Functor<B>
    where
        Standard: Distribution<R>;
}

/// A type that is enumerable and can be sampled from uniformly.
///
/// This trait requires that an implementor also implement
/// [`Distribution<Self>`], to ensure that it can be sampled from. Additionally,
/// a `sample_space` associated function must be provided.
///
/// Note that **a non-uniform distribution or a non-exhaustive sample space will
/// result in a logic error**. In particular, this means that this trait should
/// **not** be implemented for [`Option<T>`], as the probability of [`None`]
/// being sampled is 0.5, regardless of the cardinality of the sample space of
/// `T`.
///
/// # Provided Implementations
///
/// This crate provides implementations of `RandomVariable` for [`bool`],
/// [`u8`], [`i8`], [`u16`], and [`i16`].
///
/// Implementations for [`u32`] or [`i32`] would involve, at minimum, a 4 GiB
/// allocation just to enumerate the outcomes of a random process with one
/// `fmap_rand`. This is obviously intractable, so implementations are not
/// provided for any types larger than 16 bits. The Newtype pattern can be used
/// to get around this, if desired.
///
/// # Implementing `RandomVariable`
///
/// Neither `Distribution<T> for Standard` nor `RandomVariable for T` are
/// derivable. However, implementations for simple structs tends to follow a
/// pattern. [`Distribution<Self>`] implementations will typically call
/// `self.sample(rng)` for each field of the struct. `RandomVariable`
/// implementations will typically use [`Iterator::flat_map`] to create a
/// Cartesian product of all the sample spaces of the struct's fields.
/// ```
/// use rand::distributions::Standard;
/// use rand::prelude::*;
/// use rand_functors::RandomVariable;
///
/// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// struct Coordinate {
///     x: u8,
///     y: u8,
/// }
///
/// impl Distribution<Coordinate> for Standard {
///     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coordinate {
///         Coordinate {
///             x: self.sample(rng),
///             y: self.sample(rng),
///         }
///     }
/// }
///
/// impl RandomVariable for Coordinate {
///     fn sample_space() -> impl Iterator<Item = Self> {
///         u8::sample_space().flat_map(|x| u8::sample_space().map(move |y| Coordinate { x, y }))
///     }
/// }
/// ```
pub trait RandomVariable: Sized
where
    Standard: Distribution<Self>,
{
    /// Produce an [`Iterator`] containing all possible values of this type.
    ///
    /// This iterator must be finite, though a trait bound of
    /// [`ExactSizeIterator`] is not specified, to allow the use of
    /// [`Iterator::flat_map`] in implementations of this trait.
    fn sample_space() -> impl Iterator<Item = Self>;
}

/// A container used by a [`RandomStrategy`] during computations.
///
/// In functional programming, the Functor pattern allows one to apply functions
/// to values inside a container type, without changing the container's
/// structure. A Functor must support the `fmap` method, which applies the
/// function passed to it as a parameter to the contents of the Functor.
///
/// Additionally, this trait requires that implementors provide the `pure`
/// associated function. This provides for a way to begin a series of `fmap` and
/// `fmap_rand` operations. This requirement technically puts this crate's
/// functors halfway between "normal" functors and applicative functors, as a
/// subset of the former and a superset of the latter. However, implementing
/// full applicative functors would be unnecessary for the sorts of computations
/// that this crate focuses on.
pub trait Functor<I: Inner> {
    /// The functor produced by [`Functor::fmap`].
    ///
    /// This should always be the same type as `Self`, just parametrized with
    /// `O` rather than `I`.
    ///
    /// This is as workaround to ensure that a `Functor` is not restricted to
    /// being an endofunctor. The need for this is a consequence of the fact
    /// that `Self<I: Inner>` does not implement `Functor<I>` but rather `Self`
    /// implements `Functor<I: Inner>`. This means that the enclosing type
    /// cannot be accessed from the trait in signatures.
    type Output<O: Inner>: Functor<O>;

    /// Produce an instance of `Self` containing the argument as its inner.
    ///
    /// This associated function is often used to begin a series of
    /// computations. The associated functions of [`RandomStrategy`] only
    /// operate on the `Functor` associated with that [`RandomStrategy`].
    fn pure(i: I) -> Self;

    /// Applies the given function to the functor's inner.
    fn fmap<O: Inner, F: Fn(I) -> O>(self, func: F) -> Self::Output<O>;
}

/// A valid inner type for a [`Functor`].
///
/// [`Clone`] is required because most non-trivial [`Functor`] implementations
/// will need to clone their inner type. [`Eq`] and [`Hash`] are required to
/// allow for [`Functor`] implementations involving maps and sets. It was
/// determined that [`Hash`] was a less burdensome requirement than [`Ord`].
pub trait Inner: Clone + Eq + Hash + PartialEq {}

impl<T: Clone + Eq + Hash + PartialEq> Inner for T {}
