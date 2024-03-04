# rand-functors

`rand-functors` provides an abstraction over different ways of evaluating random processes expressed as functions of both deterministic and stochastic data.

A motivating problem for this crate is the code duplication present across these two functions modelling the same random process:
```rust
use rand::prelude::*;

fn next_state(mut state: u8) -> u8 {
    state = state.wrapping_add(random());
    if random() {
        state %= 3;
    }
    state
}

fn next_states(state: u8) -> Vec<u8> {
    let mut out: Vec<_> = (0..=255).map(|r| state.wrapping_add(r)).collect();
    out.append(&mut out.iter().copied().map(|i| i % 3).collect());
    out
}
```
Using `rand-functors`, these two functions can be combined as:
```rust
use rand::prelude::*;
use rand_functors::{Functor, RandomStrategy};

fn next_state<S: RandomStrategy>(state: u8) -> S::Functor<u8> {
    let mut out = S::fmap_rand(Functor::pure(state), &mut thread_rng(), |s, r| {
        s.wrapping_add(r)
    });
    out = S::fmap_rand(out, &mut thread_rng(), |s, r| if r { s % 3 } else { s });
    out
}
```
At present, `rand-functors` only supports random variables that are either of type [`bool`] or of a numeric type occupying no more than 16 bits.
