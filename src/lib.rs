//! `rand-functors` provides an abstraction over different ways of evaluating
//! random processes expressed as functions of both deterministic and stochastic
//! data.
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
//! At present, `rand-functors` only supports random variables that are either
//! of type [`bool`] or of a numeric type occupying no more than 16 bits.

#![warn(clippy::cargo)]
#![warn(missing_docs)]

pub use handlers::*;

mod functors;
mod handlers;
mod random_variables;

use std::hash::Hash;

use rand::distributions::Standard;
use rand::prelude::*;

/// A strategy for evaluating sequences of functions of random data.
pub trait RandomStrategy {
    /// The functor that this strategy operates on.
    ///
    /// Functions using a given strategy will typically return its associated
    /// functor in the form `impl S::Functor<T>`.
    type Functor<I: Inner>: Functor<I>;

    /// Using the strategy specified by the implementor, apply the given binary
    /// function to the given functor and an element of the sample space of a
    /// [`RandomVariable`].
    ///
    /// Note that **no guarantees** are made about whether or how the `rand`
    /// parameter will be used. It may be sampled zero, one, or arbitrarily many
    /// times. It may be used to sample values of type `R`, of type [`usize`],
    /// or some other type.
    fn fmap_rand<A: Inner, B: Inner, R: RandomVariable, F: Fn(A, R) -> B>(
        f: Self::Functor<A>,
        rand: &mut impl Rng,
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
    fn sample_space() -> impl Iterator<Item = Self>;
}

/// A container used by a [`RandomStrategy`] during computations.
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
